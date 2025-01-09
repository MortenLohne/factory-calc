import React, { useState } from 'react';
import MoveSelection from './MoveSelection';

const OpponentSelector = ({ pokemonData, setIncludedMons }) => {
    console.log(`Rendering opponent selector with ${pokemonData.length} pokemmon`);
    console.log(`First moveset ${pokemonData[0]?.moves}`);

    // An object with all species as keys, mapped to their movesets
    const allSpecies = pokemonData.reduce((acc, { species, moves }) => {
        if (acc[species]) {
            acc[species].movesets.push(moves);
        } else {
            acc[species] = { movesets: [moves] };
        }
        return acc;
    }, {});

    const [opponents, setOpponents] = useState([
        { species: '', possibleMonIds: [] },
        { species: '', possibleMonIds: [] },
        { species: '', possibleMonIds: [] },
    ]);

    const handleSpeciesChange = (index, event) => {
        const newOpponents = [...opponents];
        newOpponents[index] = { species: event.target.value, possibleMonIds: opponents[index].possibleMonIds };
        setOpponents(newOpponents);
        setIncludedMons(newOpponents);
    };


    const updateMovesets = (index, movesets) => {
        const newOpponents = [...opponents];
        const moveset = movesets;
        newOpponents[index].possibleMonIds = moveset;
        setOpponents(newOpponents);
        setIncludedMons(newOpponents);
    };

    const firstSpecies = Object.keys(allSpecies)[0];
    console.log(`First species ${firstSpecies} has ${allSpecies[firstSpecies]?.movesets?.length} movesets: ${allSpecies[firstSpecies]?.movesets}`);

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
                        {Object.keys(allSpecies).map((species) => (
                            <option key={species} value={species}>
                                {species}
                            </option>
                        ))}
                    </select>
                    <MoveSelection setMovesetsSelected={(newMovesets) => updateMovesets(index, newMovesets)} movesets={opponents[index].possibleMonIds}></MoveSelection>
                </div>
            ))
            }
        </div >
    );
};

export default OpponentSelector;