pub mod data;

use std::{cmp::Ordering, fmt, hash, iter, mem, str::FromStr};

use bincode::{Decode, Encode};
use data::{Ability, Item, Move, Nature, Species, Type};
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{EnumCount, EnumIter, EnumString, FromRepr};

type LookupTable = PokemonTable<Option<Pokemon>>;

type PokemonTable<T> = [[T; 10]; Species::COUNT];

use wasm_bindgen::{convert::TryFromJsValue, prelude::*};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub struct Data {
    pub(crate) pokemon: Vec<Pokemon>,
    pub(crate) lookup_table: LookupTable,
    pub(crate) all_teams: Vec<Team>,
}

#[wasm_bindgen]
impl Data {
    pub fn pokemon(&self) -> Vec<Pokemon> {
        self.pokemon.clone()
    }
    #[wasm_bindgen(constructor)]
    pub fn generate() -> Self {
        let pokemon_bin = include_bytes!("pokemon.bin");
        let pokemon: Vec<Pokemon> =
            bincode::decode_from_slice(pokemon_bin, bincode::config::standard())
                .unwrap()
                .0;

        // Each Pokemon has max 10 variants in the factory
        let mut lookup_table: LookupTable = [const { [const { None }; 10] }; Species::COUNT];
        for mon in pokemon.iter() {
            lookup_table[mon.species as usize][mon.id as usize - 1] = Some(mon.clone());
        }

        let unique_teams: Vec<Team> = unique_teams(&lookup_table);
        Self {
            pokemon,
            lookup_table,
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
        for team in self.all_teams.iter() {
            if (typ.is_none() || typ == Some(team.typ))
                && (phrase.is_none() || phrase == Some(team.phrase))
                && excluded_species
                    .iter()
                    .all(|species| team.pokemon.iter().all(|r| r.species != *species))
                && known_mons.iter().all(
                    |KnownPokemon {
                         species,
                         moves,
                         item,
                     }| {
                        team.pokemon.iter().any(|r| {
                            let team_mon = self.lookup_table[r.species as usize][r.id as usize - 1]
                                .as_ref()
                                .unwrap();
                            (item.is_none() || Some(team_mon.item) == *item)
                                && team_mon.species == *species
                                && moves.iter().all(|mv| team_mon.moves.contains(mv))
                        })
                    },
                )
            {
                matching_teams += 1;
                for pokemon in team.pokemon.iter() {
                    result[pokemon.species as usize][pokemon.id as usize - 1] += 1;
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
        known_species: Vec<String>,
        excluded_species: Vec<String>,
    ) -> Vec<PokemonProbability> {
        let typ = typ.and_then(|s| Type::from_str(&s).ok());
        let phrase = phrase.and_then(|s| Style::from_str(&s).ok());
        self.compute_mon_probs(
            typ,
            phrase,
            vec![],
            excluded_species
                .into_iter()
                .map(|s| Species::from_str(&s).unwrap())
                .collect(),
        )
        .into_iter()
        .map(|(mon, p)| PokemonProbability {
            pokemon: self.lookup_table[mon.species as usize][mon.id as usize - 1]
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

fn unique_teams(lookup_table: &LookupTable) -> Vec<Team> {
    let mut unique_teams = Vec::new();

    for (i, mon1) in lookup_table
        .iter()
        .enumerate()
        .flat_map(|(i, ids)| ids.iter().flatten().map(move |id| (i, id)))
    {
        for (j, mon2) in lookup_table
            .iter()
            .enumerate()
            .skip(i + 1)
            .flat_map(|(i, ids)| ids.iter().flatten().map(move |id| (i, id)))
        {
            for mon3 in lookup_table.iter().skip(j + 1).flatten().flatten() {
                let mons = [mon1.into(), mon2.into(), mon3.into()];

                if is_valid_team(mons, lookup_table) {
                    unique_teams.push(Team::new(mons, lookup_table));
                }
            }
        }
    }
    unique_teams
}

#[wasm_bindgen]
pub struct KnownPokemon {
    pub species: Species,
    #[wasm_bindgen(getter_with_clone)]
    pub moves: Vec<Move>,
    pub item: Option<Item>,
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
    species: Species,
    id: u8,
    styles: [Style; 4],
    moves: [Move; 4],
    item: Item,
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
    pub fn toString(&self) -> String {
        format!("{}-{}", self.species, self.id)
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
#[wasm_bindgen]
pub struct Team {
    pokemon: [PokemonRef; 3],
    phrase: Style,
    typ: Type,
}

impl Team {
    fn new(pokemon: [PokemonRef; 3], lookup_table: &LookupTable) -> Self {
        Self {
            pokemon,
            phrase: phrase(pokemon, lookup_table),
            typ: type_hint(pokemon, lookup_table),
        }
    }
}

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

fn type_hint(pokemon: [PokemonRef; 3], lookup_table: &LookupTable) -> Type {
    let mut type_map: [u8; Type::COUNT] = [0; Type::COUNT];
    for typ in pokemon.into_iter().flat_map(|p| {
        lookup_table[p.species as usize][p.id as usize - 1]
            .as_ref()
            .unwrap()
            .types
            .into_iter()
            .flatten()
    }) {
        type_map[typ as usize] += 1;
    }

    let mut highest_type_id = None;
    let mut highest_type_count = 0;

    for (type_id, count) in type_map.into_iter().enumerate() {
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

fn phrase(pokemon: [PokemonRef; 3], lookup_table: &LookupTable) -> Style {
    let mut style_map: [u8; Style::COUNT] = [0; Style::COUNT];

    for style in pokemon.iter().flat_map(|p| {
        lookup_table[p.species as usize][p.id as usize - 1]
            .as_ref()
            .unwrap()
            .styles
            .iter()
    }) {
        style_map[*style as usize] += 1;
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
