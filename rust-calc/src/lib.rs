pub mod data;

use std::{
    cmp::Ordering,
    fmt::{self, Display},
    hash, iter, mem,
    str::FromStr,
};

use bincode::{Decode, Encode};
use data::{Ability, Item, Move, Nature, Species, Type};
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{EnumCount, EnumIter, EnumString, FromRepr};

type LookupTable = PokemonTable<Option<Pokemon>>;

type PokemonTable<T> = [[T; 10]; Species::COUNT];

use wasm_bindgen::prelude::*;

use console_error_panic_hook;
use std::panic;

mod tests;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub struct PokemonData {
    pokemon: Vec<Pokemon>,
    lookup_table: LookupTable,
    types: [[Type; 2]; Species::COUNT], // Each species' types. For monotype species, the second type is Typeless
    phrases: PokemonTable<[Style; 4]>, // Each pokemon's phrases. For pokemon that don't exist (E.g. Aerodactyl-10), the phrase is Style::FreeSpirited
}

#[wasm_bindgen]
impl PokemonData {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        let pokemon_bin = include_bytes!("pokemon.bin");
        let pokemon: Vec<Pokemon> =
            bincode::decode_from_slice(pokemon_bin, bincode::config::standard())
                .unwrap()
                .0;

        // Each Pokemon has max 10 variants in the factory
        let lookup_table: LookupTable = create_lookup_table(&pokemon);

        let types = Species::iter()
            .map(|species| {
                let mon = lookup_table[species as usize][0].as_ref().unwrap();
                [
                    mon.types[0].unwrap(),
                    mon.types[1].unwrap_or(Type::Typeless),
                ]
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let mut phrases_table: PokemonTable<[Style; 4]> =
            [[[Style::FreeSpirited; 4]; 10]; Species::COUNT];
        for mon in &pokemon {
            phrases_table[mon.species as usize][mon.id as usize - 1] = mon.styles;
        }
        Self {
            pokemon,
            lookup_table,
            types,
            phrases: phrases_table,
        }
    }

    pub fn compute_wasm(
        &self,
        typ: Option<String>,
        phrase: Option<String>,
        known_first_mon: Option<KnownPokemon>,
        known_back_mons: Vec<KnownPokemon>,
        excluded_species: Vec<String>,
    ) -> Vec<PokemonProbability> {
        let typ = typ.and_then(|s| Type::from_str(&s).ok());
        let phrase = phrase.and_then(|s| Style::from_str(&s).ok());
        let excluded_species: Vec<Species> = excluded_species
            .into_iter()
            .map(|s| Species::from_str(&s).unwrap())
            .collect();

        let result = self.compute_mon_probs(
            typ,
            phrase,
            &known_first_mon,
            &known_back_mons,
            &excluded_species,
        );

        for (mon, probs) in result.iter() {
            for p in probs.iter() {
                assert!(!p.is_nan(), "Got {:?} probs for {}", probs, mon)
            }
        }

        let mut possible_pokemon: Vec<PokemonProbability> = result
            .into_iter()
            .map(|(mon, probs)| PokemonProbability {
                pokemon: self.lookup_table[mon.species as usize][mon.id as usize - 1]
                    .as_ref()
                    .unwrap()
                    .into(),
                probability: probs.into_iter().sum::<f64>() as f32,
            })
            .collect();
        possible_pokemon
            .sort_by(|a, b| a.probability.partial_cmp(&b.probability).unwrap().reverse());
        possible_pokemon
    }

    #[wasm_bindgen(js_name = allPokemon)]
    pub fn all_pokemon(&self) -> Vec<JSPokemon> {
        self.pokemon.iter().map(|p| p.into()).collect()
    }
}

impl PokemonData {
    fn type_hint(&self, pokemon: [PokemonRef; 3]) -> Type {
        let mut type_map: [u8; Type::COUNT] = [0; Type::COUNT];
        for [type1, type2] in pokemon.into_iter().map(|p| self.types[p.species as usize]) {
            type_map[type1 as usize] += 1;
            type_map[type2 as usize] += 1;
        }

        let mut highest_type_id = None;
        let mut highest_type_count = 0;

        // Skip the last type, which is Typeless
        for (type_id, count) in type_map.into_iter().enumerate().take(Type::COUNT - 1) {
            match count.cmp(&highest_type_count) {
                Ordering::Greater => {
                    highest_type_id = Some(type_id);
                    highest_type_count = count;
                }
                Ordering::Equal => highest_type_id = None,
                Ordering::Less => (),
            }
        }

        if let Some(id) = highest_type_id {
            Type::from_repr(id).unwrap()
        } else {
            Type::Typeless
        }
    }

    // Forcing the compiler to not inline makes this 10% faster for some reason
    #[inline(never)]
    fn phrase(&self, pokemon: [PokemonRef; 3]) -> Style {
        let mut style_map: [u8; Style::COUNT] = [0; Style::COUNT];

        for style in pokemon
            .iter()
            .flat_map(|p| self.phrases[p.species as usize][p.id as usize - 1])
        {
            style_map[style as usize] += 1;
        }

        let mut triggered_style_id: Option<usize> = None;
        let mut num_triggered_styles = 0;

        for (style, count) in style_map.iter().enumerate().skip(1).take(3) {
            if *count >= 3 {
                triggered_style_id = Some(style);
                num_triggered_styles += 1;
            }
        }
        for (style, count) in style_map.iter().enumerate().skip(4) {
            if *count >= 2 {
                triggered_style_id = Some(style);
                num_triggered_styles += 1;
            }
        }
        if num_triggered_styles >= 3 {
            Style::Adaptable
        } else if let Some(id) = triggered_style_id {
            Style::from_repr(id).unwrap()
        } else {
            Style::FreeSpirited
        }
    }

    pub fn compute_mon_probs(
        &self,
        typ: Option<Type>,
        phrase: Option<Style>,
        known_first_mon: &Option<KnownPokemon>,
        other_known_mons: &[KnownPokemon],
        excluded_species: &[Species],
    ) -> Vec<(PokemonRef, [f64; 3])> {
        let mut p_sum = 0.0;
        let mut p_per_pokemon: PokemonTable<[f64; 3]> = [[[0.0; 3]; 10]; Species::COUNT];

        let all_possible_mons1: Vec<Pokemon> = self
            .pokemon
            .iter()
            .filter(|mon| {
                !excluded_species.contains(&mon.species)
                    && known_first_mon.as_ref().is_none_or(|first_mon| {
                        first_mon.species == mon.species && first_mon.contains_set(mon.id)
                    })
            })
            .cloned()
            .collect();

        let num_possibilities = all_possible_mons1.len();
        for mon1 in &all_possible_mons1 {
            let all_possible_mons2 = self
                .pokemon
                .iter()
                .filter(|other_mon| {
                    !excluded_species.contains(&other_mon.species)
                        && other_mon.species != mon1.species
                        && other_mon.item != mon1.item
                        && other_known_mons.first().is_none_or(|known_mon| {
                            known_mon.species == other_mon.species
                                && known_mon.contains_set(other_mon.id)
                        })
                })
                .cloned()
                .collect::<Vec<_>>();
            let num_possibilities2 = all_possible_mons2.len();
            for mon2 in all_possible_mons2.iter() {
                let all_possible_mons3 = self
                    .pokemon
                    .iter()
                    .filter(|other_mon| {
                        !excluded_species.contains(&other_mon.species)
                            && other_mon.species != mon1.species
                            && other_mon.item != mon1.item
                            && other_mon.species != mon2.species
                            && other_mon.item != mon2.item
                            && other_known_mons.get(1).is_none_or(|known_mon| {
                                known_mon.species == other_mon.species
                                    && known_mon.contains_set(other_mon.id)
                            })
                    })
                    .cloned()
                    .collect::<Vec<_>>();
                let num_possibilities3 = all_possible_mons3.len();
                for mon3 in all_possible_mons3.iter() {
                    let team = Team::new([mon1.into(), mon2.into(), mon3.into()], self);
                    debug_assert!(is_valid_team(team.pokemon, &self.lookup_table));
                    if typ.is_none_or(|t| t == team.typ)
                        && phrase.is_none_or(|ph| ph == team.phrase)
                    {
                        let p = 1.0
                            / (num_possibilities * num_possibilities2 * num_possibilities3) as f64;
                        p_sum += p;

                        for (i, mon) in team.pokemon.iter().enumerate() {
                            p_per_pokemon[mon.species as usize][mon.id as usize - 1][i] += p;
                        }
                    }
                }
            }
        }

        // Avoid division by zero if we found zero matching teams
        if p_sum == 0.0 {
            return vec![];
        }

        // Scale all the probabilities so they sum to 1
        p_per_pokemon
            .iter_mut()
            .flatten()
            .flatten()
            .for_each(|p| *p /= p_sum);

        self.pokemon
            .iter()
            .filter_map(|pokemon| {
                let probs = p_per_pokemon[pokemon.species as usize][pokemon.id as usize - 1];
                if probs == [0.0; 3] {
                    None
                } else {
                    Some((pokemon.into(), probs))
                }
            })
            .collect()
    }

    fn unique_teams(&self) -> [[Vec<Team>; Style::COUNT]; Type::COUNT] {
        let mut unique_teams = [const { [const { Vec::new() }; Style::COUNT] }; Type::COUNT];

        for (i, mon1) in self
            .lookup_table
            .iter()
            .enumerate()
            .flat_map(|(i, ids)| ids.iter().flatten().map(move |id| (i, id)))
        {
            for (j, mon2) in self
                .lookup_table
                .iter()
                .enumerate()
                .skip(i + 1)
                .flat_map(|(i, ids)| ids.iter().flatten().map(move |id| (i, id)))
            {
                for mon3 in self.lookup_table.iter().skip(j + 1).flatten().flatten() {
                    let mons = [mon1.into(), mon2.into(), mon3.into()];

                    if is_valid_team(mons, &self.lookup_table) {
                        let team = Team::new(mons, self);
                        unique_teams[team.typ as usize][team.phrase as usize].push(team);
                    }
                }
            }
        }
        unique_teams
    }
}

pub fn create_lookup_table(all_pokemon: &[Pokemon]) -> LookupTable {
    let mut lookup_table: LookupTable = [const { [const { None }; 10] }; Species::COUNT];
    for mon in all_pokemon {
        lookup_table[mon.species as usize][mon.id as usize - 1] = Some(mon.clone());
    }
    lookup_table
}

#[wasm_bindgen]
pub struct Data {
    pub(crate) small_data: PokemonData,
    pub(crate) all_teams: [[Vec<Team>; Style::COUNT]; Type::COUNT], // All possible teams, split into buckets by type and phrase
}

#[wasm_bindgen]
impl Data {
    pub fn pokemon(&self) -> Vec<Pokemon> {
        self.small_data.pokemon.clone()
    }
    #[wasm_bindgen(constructor)]
    pub fn generate() -> Self {
        console_error_panic_hook::set_once();
        let small_data = PokemonData::new();

        let unique_teams: [[Vec<Team>; Style::COUNT]; Type::COUNT] = small_data.unique_teams();
        Self {
            small_data,
            all_teams: unique_teams,
        }
    }

    /// Returns the number of teams that match the given criteria,
    /// and a table of the number of times each moveset of each species appears in those teams
    pub(crate) fn compute_prob_table(
        &self,
        typ: Option<Type>,
        phrase: Option<Style>,
        known_mons: Vec<KnownPokemon>,
        excluded_species: Vec<Species>,
    ) -> (u32, PokemonTable<u32>) {
        let mut result: PokemonTable<u32> = [[0; 10]; Species::COUNT];
        let mut matching_teams = 0;
        for (i, teamss) in self.all_teams.iter().enumerate() {
            if typ.is_some_and(|t| t as usize != i) {
                continue;
            }
            for (j, teams) in teamss.iter().enumerate() {
                if phrase.is_some_and(|p| p as usize != j) {
                    continue;
                }
                for team in teams.iter() {
                    if excluded_species
                        .iter()
                        .all(|species| team.pokemon.iter().all(|r| r.species != *species))
                        && known_mons.iter().all(|known_mon| {
                            team.pokemon.iter().any(|team_mon| {
                                known_mon.species == team_mon.species
                                    && known_mon.contains_set(team_mon.id)
                            })
                        })
                    {
                        matching_teams += 1;
                        for pokemon in team.pokemon.iter() {
                            result[pokemon.species as usize][pokemon.id as usize - 1] += 1;
                        }
                    }
                }
            }
        }
        (matching_teams, result)
    }

    pub(crate) fn compute_mon_probs(
        &self,
        typ: Option<Type>,
        phrase: Option<Style>,
        known_mons: Vec<KnownPokemon>,
        excluded_species: Vec<Species>,
    ) -> Vec<(PokemonRef, f32)> {
        let (matching_teams, probs) =
            self.compute_prob_table(typ, phrase, known_mons, excluded_species);
        if matching_teams == 0 {
            return vec![];
        }
        let mut mons: Vec<(PokemonRef, f32)> = self
            .small_data
            .pokemon
            .iter()
            .map(|pokemon| {
                (
                    pokemon.into(),
                    probs[pokemon.species as usize][pokemon.id as usize - 1] as f32
                        / matching_teams as f32,
                )
            })
            .collect();
        mons.sort_by(|(_, p1), (_, p2)| p1.partial_cmp(p2).unwrap().reverse());
        mons
    }

    pub fn compute(
        &self,
        typ: Option<String>,
        phrase: Option<String>,
        known_pokemon: Vec<KnownPokemon>,
        excluded_species: Vec<String>,
    ) -> Vec<PokemonProbability> {
        let typ = typ.and_then(|s| Type::from_str(&s).ok());
        let phrase = phrase.and_then(|s| Style::from_str(&s).ok());
        self.compute_mon_probs(
            typ,
            phrase,
            known_pokemon,
            excluded_species
                .into_iter()
                .map(|s| Species::from_str(&s).unwrap())
                .collect(),
        )
        .into_iter()
        .map(|(mon, p)| PokemonProbability {
            pokemon: self.small_data.lookup_table[mon.species as usize][mon.id as usize - 1]
                .as_ref()
                .unwrap()
                .into(),
            probability: p,
        })
        .collect()
    }
}

pub fn compute_species_probs(mon_probs: &[(PokemonRef, f32)]) -> Vec<(Species, f32)> {
    let mut species_prob: Vec<(Species, f32)> = Species::iter()
        .map(|spec| {
            (
                spec,
                mon_probs
                    .iter()
                    .filter(|(mon, _)| mon.species == spec)
                    .map(|(_, p)| p)
                    .sum(),
            )
        })
        .collect();

    species_prob.sort_by(|(_, p1), (_, p2)| p1.partial_cmp(p2).unwrap().reverse());
    species_prob
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct KnownPokemon {
    pub species: Species,
    possible_sets_bitset: u16,
}

#[wasm_bindgen]
impl KnownPokemon {
    #[wasm_bindgen(constructor)]
    pub fn new_from_js(species: String, possible_sets: Vec<u8>) -> Self {
        let mut possible_sets_bitset: u16 = 0;
        for n in possible_sets {
            assert_ne!(n, 0);
            possible_sets_bitset |= 1 << n;
        }
        Self {
            species: species.parse().unwrap(),
            possible_sets_bitset,
        }
    }

    #[allow(non_snake_case)]
    pub fn toString(&self) -> String {
        self.to_string()
    }

    pub fn contains_set(&self, set: u8) -> bool {
        self.possible_sets_bitset & (1 << set) != 0
    }
}

impl Display for KnownPokemon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let possible_sets = (1..=10)
            .filter(|n| self.contains_set(*n))
            .collect::<Vec<_>>();
        let sets_string = possible_sets
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join("/");
        write!(f, "{}-{}", self.species, sets_string)
    }
}

#[wasm_bindgen(inspectable)]
pub struct PokemonProbability {
    #[wasm_bindgen(getter_with_clone)]
    pub pokemon: JSPokemon,
    pub probability: f32,
}

#[derive(
    Debug,
    strum_macros::Display,
    EnumIter,
    EnumCount,
    EnumString,
    FromRepr,
    PartialEq,
    Eq,
    Copy,
    Clone,
    PartialOrd,
    Ord,
    Hash,
    Encode,
    Decode,
)]
#[wasm_bindgen]
pub enum Style {
    FreeSpirited = 0,
    Preparation = 1,
    SlowAndSteady = 2,
    Endurance = 3,
    HighRiskHighReturn = 4,
    Weakening = 5,
    Unpredictable = 6,
    BattleFlow = 7,
    Adaptable = 8,
}

impl TryFrom<u8> for Style {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > 7 {
            Err(())
        } else {
            Ok(unsafe { mem::transmute::<u8, Style>(value) })
        }
    }
}

#[derive(Clone)]
#[wasm_bindgen(inspectable)]
// A Pokemon representation intended to be exported to Javascript
// It's separate from the `Pokemon` structs, because exporting enums through wasm is just too messy
pub struct JSPokemon {
    #[wasm_bindgen(getter_with_clone)]
    pub species: String,
    pub id: u8,
    #[wasm_bindgen(getter_with_clone)]
    pub styles: Vec<String>,
    #[wasm_bindgen(getter_with_clone)]
    pub moves: Vec<String>,
    #[wasm_bindgen(getter_with_clone)]
    pub item: String,
    #[wasm_bindgen(getter_with_clone)]
    pub nature: String,
    #[wasm_bindgen(getter_with_clone)]
    pub abilities: Vec<String>,
    #[wasm_bindgen(getter_with_clone)]
    pub evs: Vec<u8>,
    #[wasm_bindgen(getter_with_clone)]
    pub types: Vec<String>,
}

impl From<&Pokemon> for JSPokemon {
    fn from(pokemon: &Pokemon) -> Self {
        JSPokemon {
            species: pokemon.species.to_string(),
            id: pokemon.id,
            styles: pokemon
                .styles
                .into_iter()
                .map(|style| style.to_string())
                .collect::<Vec<_>>(),
            moves: pokemon
                .moves
                .into_iter()
                .map(|mv| mv.to_string())
                .collect::<Vec<_>>(),
            item: pokemon.item.to_string(),
            nature: pokemon.item.to_string(),
            abilities: pokemon
                .abilities
                .into_iter()
                .flatten()
                .map(|ability| ability.to_string())
                .collect(),
            evs: pokemon.evs.to_vec(),
            types: pokemon
                .types
                .into_iter()
                .flatten()
                .map(|typ| typ.to_string())
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Encode, Decode)]
#[wasm_bindgen(inspectable)]
pub struct Pokemon {
    pub species: Species,
    pub id: u8,
    styles: [Style; 4],
    moves: [Move; 4],
    pub item: Item,
    nature: Nature,
    abilities: [Option<Ability>; 2],
    evs: [u8; 6],
    types: [Option<Type>; 2],
}

impl fmt::Display for Pokemon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.species, self.id)
    }
}

impl Ord for Pokemon {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.species
            .cmp(&other.species)
            .then(self.id.cmp(&other.id))
    }
}

impl PartialOrd for Pokemon {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl hash::Hash for Pokemon {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.species.hash(state);
        self.id.hash(state);
    }
}

impl PartialEq for Pokemon {
    fn eq(&self, other: &Self) -> bool {
        self.species == other.species && self.id == other.id
    }
}

impl Eq for Pokemon {}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
#[wasm_bindgen(inspectable)]
pub struct PokemonRef {
    #[wasm_bindgen]
    pub species: Species,
    #[wasm_bindgen]
    pub id: u8,
}

#[wasm_bindgen]
impl PokemonRef {
    #[allow(non_snake_case)]
    pub fn toString(&self) -> String {
        self.to_string()
    }
}

impl From<&Pokemon> for PokemonRef {
    fn from(pokemon: &Pokemon) -> Self {
        Self {
            species: pokemon.species,
            id: pokemon.id,
        }
    }
}

impl fmt::Display for PokemonRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.species, self.id)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord, Hash)]
pub struct Team {
    pub pokemon: [PokemonRef; 3],
    phrase: Style,
    typ: Type,
}

impl Team {
    // Never inline, for profiling purposes
    #[inline(never)]
    fn new(pokemon: [PokemonRef; 3], small_data: &PokemonData) -> Self {
        Self {
            pokemon,
            phrase: small_data.phrase(pokemon),
            typ: small_data.type_hint(pokemon),
        }
    }
}

// Never inline, for profiling purposes
#[inline(never)]
fn is_valid_team(pokemon: [PokemonRef; 3], lookup_table: &LookupTable) -> bool {
    pokemon[0].species != pokemon[1].species
        && pokemon[0].species != pokemon[2].species
        && pokemon[1].species != pokemon[2].species
        && lookup_table[pokemon[0].species as usize][pokemon[0].id as usize - 1]
            .as_ref()
            .unwrap()
            .item
            != lookup_table[pokemon[1].species as usize][pokemon[1].id as usize - 1]
                .as_ref()
                .unwrap()
                .item
        && lookup_table[pokemon[0].species as usize][pokemon[0].id as usize - 1]
            .as_ref()
            .unwrap()
            .item
            != lookup_table[pokemon[2].species as usize][pokemon[2].id as usize - 1]
                .as_ref()
                .unwrap()
                .item
        && lookup_table[pokemon[1].species as usize][pokemon[1].id as usize - 1]
            .as_ref()
            .unwrap()
            .item
            != lookup_table[pokemon[2].species as usize][pokemon[2].id as usize - 1]
                .as_ref()
                .unwrap()
                .item
}

pub fn parse_pokemon_from_csv(input: &str) -> Vec<Pokemon> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let sections: Vec<&str> = line.split(',').collect();
            let styles = sections[0]
                .split_whitespace()
                .map(|word| word.parse::<u8>().unwrap().try_into().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            let species = Species::from_str(sections[1]).unwrap();
            let id = if sections[2] == "X" {
                10
            } else {
                sections[2].parse().unwrap()
            };
            let nature = Nature::from_str(sections[3]).unwrap();
            let item = Item::from_str(sections[4]).expect(sections[4]);
            let moves = sections[5..=8]
                .iter()
                .map(|word| Move::from_str(word).unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            let evs = sections[10]
                .split('/')
                .map(|s| s.trim().parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
                .try_into()
                .unwrap();
            let abilities = sections[9]
                .split('/')
                .map(|s| Some(Ability::from_str(s.trim()).expect(s)))
                .chain(iter::once(None))
                .take(2)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();

            let types = sections[12]
                .split_whitespace()
                .map(|s| {
                    Some(Type::from_str(s).unwrap_or_else(|v| panic!("Failed to parse type {}", v)))
                })
                .chain(iter::once(None))
                .take(2)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            Pokemon {
                species,
                id,
                styles,
                moves,
                item,
                nature,
                abilities,
                evs,
                types,
            }
        })
        .collect()
}
