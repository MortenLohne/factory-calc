import React, { useState } from "react";

const SingleOpponentSelector = ({ setOpponent, pokemonData }) => {
    const [species, setSpecies] = useState("");
    // State to track the toggled state of each button. Button 1 is index 1
    const [toggledButtons, setToggledButtons] = useState(Array(11).fill(true));

    // An object with all species as keys, mapped to their movesets
    const allSpecies = pokemonData.reduce((acc, { species, moves }) => {
        if (acc[species]) {
            acc[species].movesets.push(moves);
        } else {
            acc[species] = { movesets: [moves] };
        }
        return acc;
    }, {});

    const movesets = allSpecies[species]?.movesets ?? [];

    const updateOpponents = (newSpecies, newToggledButtons) => {
        const newMovesets = allSpecies[newSpecies]?.movesets ?? [];
        setOpponent(newMovesets
            .map((_, index) => ({ species: newSpecies, id: index + 1 }))
            .filter(({ species, id }) => newToggledButtons[id]))
    };

    // Call this to set a new species, not `useSpecies` directly
    const updateSpecies = (species) => {
        setSpecies(species);
        setToggledButtons(Array(11).fill(true)); // Show all movesets by default
        updateOpponents(species, Array(11).fill(true));
    };

    // Call this to toggle a specific button, not `toggleButton` directly
    const updateToggledButtons = (label) => {
        let newToggledButtons = toggledButtons.slice();
        newToggledButtons[label] = !newToggledButtons[label];
        setToggledButtons(newToggledButtons);
        updateOpponents(species, newToggledButtons);
    }

    // Function to toggle a button's state
    const toggleButton = (index) => {
        setToggledButtons((prevState) => {
            const newState = [...prevState];
            newState[index] = !newState[index];
            return newState;
        });
    };

    const buttonStyle = (label) => {
        if (label > movesets.length) {
            return "grey"
        } else if (toggledButtons[label]) {
            return "lightblue"
        } else {
            return "pink"
        }
    }

    return (
        <div>
            <select
                value={species}
                onChange={(event) => updateSpecies(event.target.value)}
            >
                <option value="">Select species</option>
                {Object.keys(allSpecies).map((species) => (
                    <option key={species} value={species}>
                        {species}
                    </option>
                ))}
            </select>
            <div style={styles.gridContainer}>

                {/* Render buttons in a 3x4 grid */}
                {[1, 2, 3, 4, 5, 6, 7, 8, 9, ""].map((label) => (
                    <div key={label} style={styles.gridItem}>
                        {label !== "" && (
                            <button
                                onClick={() => label <= movesets.length && updateToggledButtons(label)}
                                style={{
                                    ...styles.button,
                                    backgroundColor: buttonStyle(label),
                                    cursor: label > movesets.length ? "not-allowed" : "pointer"
                                }}
                            >
                                {label}
                            </button>
                        )}
                    </div>
                ))}
                <div style={styles.gridItem}>
                    <button
                        onClick={() => movesets.length >= 10 && updateToggledButtons(10)}
                        style={{
                            ...styles.button,
                            backgroundColor: buttonStyle(10),
                            cursor: movesets.length < 10 ? "not-allowed" : "pointer"
                        }}
                    >
                        10
                    </button>
                </div>
            </div>
        </div>
    );
};

const styles = {
    gridContainer: {
        display: "grid",
        gridTemplateColumns: "repeat(3, 1fr)",
        gap: "5px",
        width: "100px",
        margin: "10px auto",
    },
    gridItem: {
        display: "flex",
        justifyContent: "center",
        alignItems: "center",
    },
    button: {
        width: "30px",
        height: "30px",
        fontSize: "18px",
        cursor: "pointer",
        border: "1px solid #ccc",
        borderRadius: "5px",
        outline: "none",
    },
};

export default SingleOpponentSelector;
