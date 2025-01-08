use bincode::{self};
use rust_calc::SmallData;
// use rust_calc::{
//     calculate_team_odds,
//     data::{Item, Species},
//     parse_pokemon_from_csv, Data, Pokemon, PokemonRef, SmallData, Team,
// };
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    io::stdout,
    mem, time,
};

// NOTE: You must comment out the `crate-type = ["cdylib"]` line from Cargo.toml to compile this
fn main() {
    // print_encoded_pokemon();

    let small_data = SmallData::new();
    let result = small_data.compute_mon_probs(None, None, &None, &[], &[]);
    println!("{:?}", result[0])

    // let SmallData {
    //     pokemon: all_pokemon,
    //     lookup_table,
    // } = SmallData::new();
    // let start_time = time::Instant::now();

    // // let all_pokemon = vec![
    // //     lookup_table[Species::Marowak as usize][0].clone().unwrap(),
    // //     lookup_table[Species::Marowak as usize][1].clone().unwrap(),
    // //     lookup_table[Species::Rapidash as usize][0].clone().unwrap(),
    // //     lookup_table[Species::Rapidash as usize][1].clone().unwrap(),
    // //     lookup_table[Species::Hypno as usize][0].clone().unwrap(),
    // //     lookup_table[Species::Hypno as usize][2].clone().unwrap(),
    // //     lookup_table[Species::Vaporeon as usize][0].clone().unwrap(),
    // //     lookup_table[Species::Vaporeon as usize][3].clone().unwrap(),
    // // ];

    // // for mon in all_pokemon.iter() {
    // //     println!(
    // //         "{:12} ({}), compatible with {} other mons",
    // //         mon.to_string(),
    // //         mon.item,
    // //         all_pokemon
    // //             .iter()
    // //             .filter(|other_mon| mon.species != other_mon.species && mon.item != other_mon.item)
    // //             .count()
    // //     );
    // // }

    // let mut team_odds = calculate_team_odds(&all_pokemon, &lookup_table, &[]);

    // // Check that all teams are unique
    // // assert_eq!(
    // //     <HashSet<Team>>::from_iter(team_odds.iter().map(|(team, _)| team.clone())).len(),
    // //     team_odds.len()
    // // );
    // println!("Got {} unique teams", team_odds.len());

    // println!(
    //     "Generated odds for {} teams, sums to {:.8}%, took {:.1}s",
    //     team_odds.len(),
    //     team_odds.iter().map(|(_, odds)| odds).sum::<f64>() * 100.0,
    //     start_time.elapsed().as_secs_f32()
    // );

    // println!(
    //     "Most likely team: {:?}",
    //     team_odds
    //         .iter()
    //         .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
    //         .unwrap()
    // );
    // println!(
    //     "Least likely team: {:?}",
    //     team_odds
    //         .iter()
    //         .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
    //         .unwrap()
    // );

    // let mut p_for_sets: BTreeMap<PokemonRef, f64> =
    //     BTreeMap::from_iter(all_pokemon.iter().map(|p| (p.into(), 0.0)));

    // let mut p_for_first_mon: BTreeMap<PokemonRef, f64> =
    //     BTreeMap::from_iter(all_pokemon.iter().map(|p| (p.into(), 0.0)));

    // let mut p_for_second_mon: BTreeMap<PokemonRef, f64> =
    //     BTreeMap::from_iter(all_pokemon.iter().map(|p| (p.into(), 0.0)));

    // let mut p_for_last_mon: BTreeMap<PokemonRef, f64> =
    //     BTreeMap::from_iter(all_pokemon.iter().map(|p| (p.into(), 0.0)));

    // for (team, p) in team_odds.iter().rev() {
    //     *p_for_first_mon.get_mut(&team.pokemon[0]).unwrap() += p;
    //     *p_for_second_mon.get_mut(&team.pokemon[1]).unwrap() += p;
    //     *p_for_last_mon.get_mut(&team.pokemon[2]).unwrap() += p;
    //     for mon in team.pokemon.iter() {
    //         *p_for_sets.get_mut(mon).unwrap() += p;
    //     }
    // }

    // let mut p_for_sets_vec: Vec<(PokemonRef, f64)> =
    //     p_for_sets.iter().map(|(r, p)| (r.clone(), *p)).collect();

    // p_for_sets_vec.sort_by(|(_, p1), (_, p2)| p1.partial_cmp(p2).unwrap().reverse());

    // let mut p_for_first_mon_vec = p_for_first_mon.into_iter().collect::<Vec<_>>();

    // p_for_first_mon_vec.sort_by(|(_, p1), (_, p2)| p1.partial_cmp(p2).unwrap().reverse());

    // let mut p_for_second_mon_vec = p_for_second_mon.into_iter().collect::<Vec<_>>();

    // p_for_second_mon_vec.sort_by(|(_, p1), (_, p2)| p1.partial_cmp(p2).unwrap().reverse());

    // let mut p_for_last_mon_vec = p_for_last_mon.into_iter().collect::<Vec<_>>();

    // p_for_last_mon_vec.sort_by(|(_, p1), (_, p2)| p1.partial_cmp(p2).unwrap().reverse());

    // let p_for_sets_sum = p_for_sets_vec.iter().map(|(_, p)| p).sum::<f64>();

    // println!("Total probability: {:.8}%", p_for_sets_sum * 100.0);

    // println!("Most likely to appear anywhere in a team:");
    // for (mon, p) in p_for_sets_vec.iter().take(30) {
    //     println!(
    //         "{:15}: {:.5}%, appears in {} teams, {:.6}% of all teams",
    //         mon.to_string(),
    //         p * 100.0,
    //         team_odds
    //             .iter()
    //             .filter(|(team, _)| team.pokemon.contains(mon))
    //             .count(),
    //         team_odds
    //             .iter()
    //             .filter(|(team, _)| team.pokemon.contains(mon))
    //             .count() as f32
    //             / team_odds.len() as f32
    //             * 100.0
    //     )
    // }

    // println!();
    // println!("Least likely to appear anywhere in a team:");

    // for (mon, p) in p_for_sets_vec.iter().rev().take(30).rev() {
    //     println!(
    //         "{:15}: {:.5}%, appears in {} teams",
    //         mon.to_string(),
    //         p * 100.0,
    //         team_odds
    //             .iter()
    //             .filter(|(team, _)| team.pokemon.contains(mon))
    //             .count()
    //     )
    // }

    // println!();
    // println!("Most likely Lum Berry with 4 total sets to appear anywhere in a team:");

    // for (mon, p) in p_for_sets_vec.iter().filter(|(mon, _)| {
    //     lookup_table[mon.species as usize].iter().flatten().count() == 4
    //         && lookup_table[mon.species as usize][mon.id as usize - 1]
    //             .as_ref()
    //             .unwrap()
    //             .item
    //             == Item::LumBerry
    // }) {
    //     println!(
    //         "{:15}: {:.6}%, appears in {} teams",
    //         mon.to_string(),
    //         p * 100.0,
    //         team_odds
    //             .iter()
    //             .filter(|(team, _)| team.pokemon.contains(mon))
    //             .count()
    //     )
    // }

    // println!();
    // println!("Least likely mons with 4 total sets to appear anywhere in a team:");

    // for (mon, p) in p_for_sets_vec
    //     .iter()
    //     .rev()
    //     .filter(|(mon, _)| lookup_table[mon.species as usize].iter().flatten().count() == 4)
    //     .take(50)
    // {
    //     println!(
    //         "{:15}: {:.6}%, appears in {} teams, {:.6}% of all teams, {:.6}% first slot, {:.6}% second slot",
    //         mon.to_string(),
    //         p * 100.0,
    //         team_odds
    //             .iter()
    //             .filter(|(team, _)| team.pokemon.contains(mon))
    //             .count(),
    //         team_odds
    //             .iter()
    //             .filter(|(team, _)| team.pokemon.contains(mon))
    //             .count() as f32
    //             / team_odds.len() as f32
    //             * 100.0,
    //         team_odds
    //             .iter()
    //             .filter(|(team, _)| team.pokemon[0] == *mon)
    //             .map(|(_, p)| p)
    //             .sum::<f64>() * 100.0,
    //         team_odds
    //             .iter()
    //             .filter(|(team, _)| team.pokemon[1] == *mon)
    //             .map(|(_, p)| p)
    //             .sum::<f64>() * 100.0,
    //     )
    // }

    // println!();
    // println!("Most likely species to appear anywhere in a team:");
    // for species in [
    //     Species::Dragonite,
    //     Species::Tyranitar,
    //     Species::Machamp,
    //     Species::Latias,
    //     Species::Salamence,
    //     Species::Latios,
    //     Species::Lapras,
    //     Species::Ursaring,
    //     Species::Gardevoir,
    //     Species::Starmie,
    //     Species::Snorlax,
    //     Species::Metagross,
    //     Species::Gengar,
    //     Species::Moltres,
    //     Species::Entei,
    //     Species::Raikou,
    //     Species::Registeel,
    // ] {
    //     println!(
    //         "{:10}: {:.5}% chance of appearing in a team",
    //         species.to_string(),
    //         lookup_table[species as usize]
    //             .iter()
    //             .flatten()
    //             .map(|mon| p_for_sets[&PokemonRef {
    //                 species,
    //                 id: mon.id
    //             }]
    //                 .clone())
    //             .sum::<f64>()
    //             * 100.0
    //     );
    // }

    // println!();

    // println!("Most likely to appear in the 1st slot:");
    // for (mon, p) in p_for_first_mon_vec.iter().take(10) {
    //     println!(
    //         "{:15}: {:.5}%, appears as 1st mon in {} teams",
    //         mon.to_string(),
    //         p * 100.0,
    //         team_odds
    //             .iter()
    //             .filter(|(team, _)| team.pokemon[0] == *mon)
    //             .count()
    //     )
    // }

    // println!();

    // for (mon, p) in p_for_first_mon_vec.iter().rev().take(10).rev() {
    //     println!("{:15}: {:.5}%", mon.to_string(), p * 100.0)
    // }

    // println!();

    // println!("Most likely to appear in the 2nd slot:");
    // for (mon, p) in p_for_second_mon_vec.iter().take(30) {
    //     println!(
    //         "{:15}: {:.5}%, appears as 2nd mon in {} teams",
    //         mon.to_string(),
    //         p * 100.0,
    //         team_odds
    //             .iter()
    //             .filter(|(team, _)| team.pokemon[1] == *mon)
    //             .count()
    //     );
    // }

    // println!();

    // for (mon, p) in p_for_second_mon_vec.iter().rev().take(30).rev() {
    //     println!(
    //         "{:15}: {:.5}%, appears as 2nd mon in {} teams",
    //         mon.to_string(),
    //         p * 100.0,
    //         team_odds
    //             .iter()
    //             .filter(|(team, _)| team.pokemon[1] == *mon)
    //             .count()
    //     )
    // }

    // println!();

    // println!("Most likely to appear in the 3rd slot:");
    // for (mon, p) in p_for_last_mon_vec.iter().take(20) {
    //     println!(
    //         "{:15}: {:.5}%, appears as 3rd mon in {} teams",
    //         mon.to_string(),
    //         p * 100.0,
    //         team_odds
    //             .iter()
    //             .filter(|(team, _)| team.pokemon[2] == *mon)
    //             .count()
    //     )
    // }

    // println!();

    // for (mon, p) in p_for_last_mon_vec.iter().rev().take(20).rev() {
    //     println!(
    //         "{:15}: {:.5}%, appears as 3rd mon in {} teams",
    //         mon.to_string(),
    //         p * 100.0,
    //         team_odds
    //             .iter()
    //             .filter(|(team, _)| team.pokemon[2] == *mon)
    //             .count()
    //     )
    // }

    // team_odds.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap().reverse());
    // for (team, p) in team_odds.into_iter().take(10) {
    //     println!(
    //         "{:11}, {:11}, {:11}: {:.8}%",
    //         team.pokemon[0].to_string(),
    //         team.pokemon[1].to_string(),
    //         team.pokemon[2].to_string(),
    //         p * 100.0
    //     );
    // }
}

// fn print_encoded_pokemon() {
//     let contents = include_str!("pokemon.csv");
//     let pokemon = parse_pokemon_from_csv(contents);

//     let pokemon_size = mem::size_of::<Pokemon>();
//     eprintln!(
//         "{} bytes total, {} per {} pokemon",
//         pokemon_size * pokemon.len(),
//         pokemon_size,
//         pokemon.len()
//     );

//     let config = bincode::config::standard();
//     bincode::encode_into_std_write(&pokemon, &mut stdout(), config).unwrap();
// }
