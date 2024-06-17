// src/FibonacciCalculator.js
import React, { useState, useEffect } from 'react';
import Worker from './calc.worker.js';

const types = [
    "Normal",
    "Fire",
    "Water",
    "Electric",
    "Grass",
    "Ice",
    "Fighting",
    "Poison",
    "Ground",
    "Flying",
    "Psychic",
    "Bug",
    "Rock",
    "Ghost",
    "Dragon",
    "Dark",
    "Steel"
];

const phrases = [
    "FreeSpirited",
    "Preparation",
    "SlowAndSteady",
    "Endurance",
    "HighRiskHighReturn",
    "Weakening",
    "Unpredictable",
    "BattleFlow",
    "Adaptable",
]


const Calculator = () => {
  const [type, setType] = useState("");
  const [phrase, setPhrase] = useState("");
  const [result, setResult] = useState(null);
  const [worker] = useState(() => new Worker());

  const handleTypeChange = (e) => {
    setType(e.target.value);
  };

  const handlePhraseChange = (e) => {
    setPhrase(e.target.value);
  };

  // Sends this initial message once during load
  // eslint-disable-next-line react-hooks/exhaustive-deps
  useEffect(() => {
    worker.postMessage(JSON.stringify({type: "Typeless", phrase: "FreeSpirited"}));
  }, []);

  useEffect(() => {
    worker.postMessage(JSON.stringify({type: type, phrase: phrase}));
    }, [type, phrase, worker]);

  worker.onmessage = (e) => {
    setResult(JSON.parse(e.data));
    console.log(JSON.parse(e.data))
  };

  return (
    <div>
      <h1> Calculator</h1>
      <select value={type} onChange={handleTypeChange}>
        <option value="">Unknown type</option>
        <option value="Typeless">No type</option>
        {types.map(type => 
            <option value={type}>{type}</option>
        )}
        
        </select>
      <select value={phrase} onChange={handlePhraseChange}>
        <option value="">Unknown phrase</option>
        {phrases.map(phrase => 
            <option value={phrase}>{phrase}</option>
        )}
        
        </select>
      {result === null ? <p>Loading Pokemon data...</p> : result.length === 0 ? <p>No matching Pokemon found!</p> : <table>
        <caption>
            Result 
        </caption>
        <thead>
            <tr>
            <th scope="col">Pokemon</th>
            <th scope="col">Probability</th>
            </tr>
        </thead>
        <tbody>
            { result.map(res => 
            <tr key = {res.name}>
                <th scope="row">{res.name}</th>
                <td>{(res.p * 100).toFixed(2) + "%"}</td>
            </tr>
            
            ) }
        </tbody>
        </table>}
    </div>
  );
};

export default Calculator;
