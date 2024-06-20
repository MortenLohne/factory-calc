use bincode::{self};
use rust_calc::{parse_pokemon_from_csv, Data, Pokemon};
use std::{io::stdout, mem};

// NOTE: You must comment out the `crate-type = ["cdylib"]` line from Cargo.toml to compile this
fn main() {
    print_encoded_pokemon();
}

fn print_encoded_pokemon() {
    let contents = include_str!("pokemon.csv");
    let pokemon = parse_pokemon_from_csv(contents);

    let pokemon_size = mem::size_of::<Pokemon>();
    eprintln!(
        "{} bytes total, {} per {} pokemon",
        pokemon_size * pokemon.len(),
        pokemon_size,
        pokemon.len()
    );

    let config = bincode::config::standard();
    bincode::encode_into_std_write(&pokemon, &mut stdout(), config).unwrap();
}
