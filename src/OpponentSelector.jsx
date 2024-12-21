import React, { useState } from 'react';

const speciesList = [
    "Aerodactyl",
    "Aggron",
    "Alakazam",
    "Altaria",
    "Ampharos",
    "Arcanine",
    "Armaldo",
    "Articuno",
    "Blastoise",
    "Blaziken",
    "Blissey",
    "Breloom",
    "Charizard",
    "Claydol",
    "Clefable",
    "Cradily",
    "Crobat",
    "Dewgong",
    "Dodrio",
    "Donphan",
    "Dragonite",
    "Dugtrio",
    "Dusclops",
    "Electabuzz",
    "Electrode",
    "Entei",
    "Espeon",
    "Exeggutor",
    "Exploud",
    "Fearow",
    "Feraligatr",
    "Flareon",
    "Flygon",
    "Forretress",
    "Gardevoir",
    "Gengar",
    "Glalie",
    "Golduck",
    "Golem",
    "Granbull",
    "Gyarados",
    "Hariyama",
    "Heracross",
    "Houndoom",
    "Hypno",
    "Jolteon",
    "Jynx",
    "Kangaskhan",
    "Kingdra",
    "Lanturn",
    "Lapras",
    "Latias",
    "Latios",
    "Ludicolo",
    "Machamp",
    "Magmar",
    "Manectric",
    "Marowak",
    "Medicham",
    "Meganium",
    "Metagross",
    "Milotic",
    "Miltank",
    "Misdreavus",
    "Moltres",
    "MrMime",
    "Muk",
    "Nidoking",
    "Nidoqueen",
    "Ninetales",
    "Porygon2",
    "Quagsire",
    "Raichu",
    "Raikou",
    "Rapidash",
    "Regice",
    "Regirock",
    "Registeel",
    "Rhydon",
    "Salamence",
    "Sceptile",
    "Scizor",
    "Shiftry",
    "Shuckle",
    "Skarmory",
    "Slaking",
    "Slowbro",
    "Slowking",
    "Snorlax",
    "Starmie",
    "Steelix",
    "Suicune",
    "Swampert",
    "Tauros",
    "Tentacruel",
    "Typhlosion",
    "Tyranitar",
    "Umbreon",
    "Ursaring",
    "Vaporeon",
    "Venusaur",
    "Victreebel",
    "Vileplume",
    "Wailord",
    "Walrein",
    "Weezing",
    "Whiscash",
    "Xatu",
    "Zapdos",
];

const OpponentSelector = ({ setIncludedSpecies }) => {
    const [opponents, setOpponents] = useState([
        { species: '', moveset: [] },
        { species: '', moveset: [] },
        { species: '', moveset: [] },
    ]);

    const handleSpeciesChange = (index, event) => {
        const newOpponents = [...opponents];
        newOpponents[index] = { species: event.target.value, moveset: opponents[index].moveset };
        setOpponents(newOpponents);
        setIncludedSpecies(newOpponents.map(opponent => opponent.species).filter(species => species));
    };


    const handleMovesetChange = (index, event) => {
        const newOpponents = [...opponents];
        const moveset = Array.from(event.target.selectedOptions, option => option.value);
        newOpponents[index].moveset = moveset;
        setOpponents(newOpponents);
    };

    return (
        <div style={{ display: 'flex', justifyContent: 'space-around' }}>
            {opponents.map((opponent, index) => (
                <div key={index}>
                    <h4>Opponent {index + 1}</h4>
                    <select
                        key={index}
                        value={opponents[index].species || ""}
                        onChange={(e) => handleSpeciesChange(index, e)}
                    >
                        <option value="">Select species</option>
                        {speciesList.map((species) => (
                            <option key={species} value={species}>
                                {species}
                            </option>
                        ))}
                    </select>
                    {/* <div>
                        <label>
                            Moveset:
                            <select
                                multiple
                                value={opponent.moveset}
                                onChange={(event) => handleMovesetChange(index, event)}
                            >
                                {[...Array(10).keys()].map(num => (
                                    <option key={num + 1} value={num + 1}>
                                        {num + 1}
                                    </option>
                                ))}
                            </select>
                        </label>
                    </div> */}
                </div>
            ))}
        </div>
    );
};

export default OpponentSelector;