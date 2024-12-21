import React from 'react';
import './ExcludedSpeciesSelector.css'; // Assuming you have a CSS file for styles

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

const ExcludedSpeciesSelector = ({ setExcludedSpecies }) => {
    const [excludedLocal, setExcludedLocal] = React.useState([null, null, null, null, null, null]);

    const handleSpeciesChange = (index, e) => {
        let clonedLocal = [...excludedLocal];
        clonedLocal[index] = e.target.value;
        setExcludedLocal(clonedLocal);

        setExcludedSpecies(clonedLocal.filter((species) => species));
    };

    return (
        <div className="excluded-species-selector">
            {Array.from({ length: 6 }).map((_, index) => (
                <select
                    key={index}
                    value={excludedLocal[index] || ""}
                    onChange={(e) => handleSpeciesChange(index, e)}
                >
                    <option value="">Select species</option>
                    {speciesList.map((species) => (
                        <option key={species} value={species}>
                            {species}
                        </option>
                    ))}
                </select>
            ))}
        </div>
    );
};
export default ExcludedSpeciesSelector;