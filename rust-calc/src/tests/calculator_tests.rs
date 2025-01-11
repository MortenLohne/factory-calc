use crate::{
    data::{Species, Type},
    Data, KnownPokemon, PokemonData, PokemonRef, Style,
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
pub fn fast_no_info_test() {
    let data = Data::generate();
    let result = data.compute_mon_probs(None, None, vec![], vec![]);
    assert!(
        result[0].0
            == PokemonRef {
                species: Species::Muk,
                id: 1
            },
        "Expected Muk-1, got {:?}",
        result[0].0
    );
    assert!((result[0].1 - 0.0072206).abs() < 0.000001);
}

#[test]
pub fn fast_known_tyranitar_test() {
    let data = Data::generate();
    let result = data.compute_mon_probs(
        Some(Type::Typeless),
        Some(Style::FreeSpirited),
        vec![KnownPokemon::new_from_js(
            Species::Tyranitar.to_string(),
            vec![2, 4, 6, 8, 10],
        )],
        vec![],
    );

    assert_eq!(
        result[0].0,
        PokemonRef {
            species: Species::Tyranitar,
            id: 4
        },
    );
    assert!((result[0].1 - 0.2686894).abs() < 0.000001);

    assert_eq!(
        result[1].0,
        PokemonRef {
            species: Species::Tyranitar,
            id: 2
        },
    );
    assert!((result[1].1 - 0.2473381).abs() < 0.000001);

    assert_eq!(
        result[7].0,
        PokemonRef {
            species: Species::Arcanine,
            id: 1
        },
    );
    assert!((result[7].1 - 0.0089900).abs() < 0.000001);
}

#[test]
pub fn calculator_test1() {
    let small_data = PokemonData::new();
    let mut result = small_data.compute_mon_probs(
        Some(Type::Bug),
        Some(Style::FreeSpirited),
        &Some(KnownPokemon::new_from_js(
            Species::Heracross.to_string(),
            vec![1, 2, 3, 4],
        )),
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
    assert!(
        (result[0].1.into_iter().sum::<f64>() - 0.3829).abs() < 0.0001,
        "Expected Heracross-3 to be 38.29% likely, got {:.1}%",
        result[0].1.into_iter().sum::<f64>() * 100.0
    );

    assert!(
        result[4].0
            == PokemonRef {
                species: Species::Armaldo,
                id: 2
            }
    );
    assert!((result[4].1.into_iter().sum::<f64>() - 0.1438).abs() < 0.0001);
}
