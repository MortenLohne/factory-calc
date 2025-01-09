import React, { useState } from "react";
import "./MoveSelection.css"; // Optional CSS file for styling

const MoveSelection = ({ setMovesetsSelected, movesets }) => {
    // State to track the toggled state of each button
    const [toggledButtons, setToggledButtons] = useState(Array(11).fill(true));

    // Function to toggle a button's state
    const toggleButton = (index) => {
        setToggledButtons((prevState) => {
            const newState = [...prevState];
            newState[index] = !newState[index];
            let movesetIndexesSelected = [];
            newState.forEach((value, index) => {
                if (value && index > 0 && index < movesets.length + 1) {
                    movesetIndexesSelected.push(index);
                }
            });
            setMovesetsSelected(movesetIndexesSelected);
            console.log(`Selected movesets: ${movesetIndexesSelected} from ${movesets.length} total movesets`);
            return newState;
        });
    };

    return (
        <div style={styles.gridContainer}>
            {/* Render buttons in a 3x4 grid */}
            {[1, 2, 3, 4, 5, 6, 7, 8, 9, ""].map((label, index) => (
                <div key={index} style={styles.gridItem}>
                    {label !== "" && (
                        <button
                            onClick={() => toggleButton(label)}
                            style={{
                                ...styles.button,
                                backgroundColor: toggledButtons[label === 0 ? 0 : label] ? "lightblue" : "white",
                            }}
                        >
                            {label}
                        </button>
                    )}
                </div>
            ))}
            <div style={styles.gridItem}>
                <button
                    onClick={() => toggleButton(10)}
                    style={{
                        ...styles.button,
                        backgroundColor: toggledButtons[10] ? "lightblue" : "white",
                    }}
                >
                    10
                </button>
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

export default MoveSelection;
