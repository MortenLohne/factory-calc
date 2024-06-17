use arrayvec::ArrayVec;
use std::time::Instant;
use strum::EnumCount;

use crate::data::{Species, Type};
use rse_bot::{compute_species_probs, Data, Style};

fn main() {
    let start_time = Instant::now();
    let data = Data::generate();

    println!(
        "Generated all {} unique teams in {:.2}s",
        data.all_teams.len(),
        start_time.elapsed().as_secs_f32()
    );

    let start_time = Instant::now();

    let mut frequencies: [[u64; Style::COUNT]; Type::COUNT] = [[0; Style::COUNT]; Type::COUNT];

    for team in data.all_teams.iter() {
        frequencies[team.typ as usize][team.phrase as usize] += 1;
    }

    println!("Computed frequencies in {:.2}s", start_time.elapsed().as_secs_f32());

    // for typ in Type::iter() {
    //     for style in Style::iter() {
    //         let count = frequencies[typ as usize][style as usize];
    //         println!(
    //             "{:?}, {}: {}, {:.3}%",
    //             typ,
    //             style,
    //             count,
    //             100.0 * count as f32 / unique_teams.len() as f32
    //         )
    //     }
    // }

    let start_time = Instant::now();
    let typ: Option<Type> = Some(Type::Typeless);
    let phrase: Option<Style> = Some(Style::FreeSpirited);

    println!(
        "Probabilities for {}, {}:",
        typ.map(|t| t.to_string()).unwrap_or("no type".to_string()),
        phrase.map(|t| t.to_string()).unwrap_or("no phrase".to_string()),
    );
    let mons = data.compute_mon_probs(
        typ,
        phrase,
        &[
            (Species::Aerodactyl, ArrayVec::new(), None),
            (Species::Alakazam, ArrayVec::new(), None),
        ],
    );

    println!("Most likely mons:");
    for (mon, p) in mons.iter().take(10) {
        println!("{}: {:.4}%", mon, p * 100.0)
    }

    println!("Computed pokemon probs in {:.2}s", start_time.elapsed().as_secs_f32());

    let start_time = Instant::now();
    let species_prob = compute_species_probs(&mons);

    println!("Most likely species:");
    for (spec, p) in species_prob.iter().take(10) {
        println!("{}: {:.4}%", spec, p * 100.0)
    }
    println!("Computed species probs in {:.2}s", start_time.elapsed().as_secs_f32());
}
