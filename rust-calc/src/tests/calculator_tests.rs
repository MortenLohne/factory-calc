use crate::{
    data::{Species, Type},
    KnownPokemon, PokemonData, PokemonRef, Style,
};

#[test]
pub fn calculator_no_info_test() {
    let small_data = PokemonData::new();
    let mut result = small_data.compute_mon_probs(None, None, &None, &[], &[]);

    result.sort_by(|(_, probs1), (_, probs2)| {
        probs1
            .iter()
            .sum::<f64>()
            .partial_cmp(&probs2.iter().sum::<f64>())
            .unwrap()
            .reverse()
    });

    assert!(
        result[0].0.species == Species::Marowak,
        "Expected any Marowak set, got {:?}",
        result[0].0
    );
    assert!((result[0].1.into_iter().sum::<f64>() - 0.006565).abs() < 0.000001);

    assert!(
        result[4].0
            == PokemonRef {
                species: Species::Dugtrio,
                id: 1
            },
        "Expected Dugtrio-1, got {:?}",
        result[0].0
    );
    assert!((result[4].1.into_iter().sum::<f64>() - 0.006564).abs() < 0.000001);
}

#[test]
pub fn calculator_test1() {
    let small_data = PokemonData::new();
    let mut result = small_data.compute_mon_probs(
        Some(Type::Bug),
        Some(Style::FreeSpirited),
        &Some(KnownPokemon {
            species: Species::Heracross,
            moves: vec![],
            item: None,
        }),
        &[],
        &vec![Species::Scizor],
    );

    result.sort_by(|(_, probs1), (_, probs2)| {
        probs1
            .iter()
            .sum::<f64>()
            .partial_cmp(&probs2.iter().sum::<f64>())
            .unwrap()
            .reverse()
    });

    assert!(
        result[0].0
            == PokemonRef {
                species: Species::Heracross,
                id: 3
            },
        "Expected Heracross-3, got {:?}",
        result[0].0
    );
    assert!((result[0].1.into_iter().sum::<f64>() - 0.3829).abs() < 0.0001);

    assert!(
        result[4].0
            == PokemonRef {
                species: Species::Armaldo,
                id: 2
            }
    );
    assert!((result[4].1.into_iter().sum::<f64>() - 0.1438).abs() < 0.0001);
}
