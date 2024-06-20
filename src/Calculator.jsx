// src/FibonacciCalculator.js
import React, { useState, useEffect } from 'react';
import './Calculator.css'; // Assuming you have a CSS file for styles
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

  useEffect(() => {
    worker.postMessage(JSON.stringify({type: type, phrase: phrase}));
    }, [type, phrase, worker]);

  worker.onmessage = (e) => {
    setResult(JSON.parse(e.data));
  };

  const probabilityPerSpecies = (result || []).reduce((acc, {pokemon, probability}) => {
    acc[pokemon.species] = (acc[pokemon.species] || 0) + probability
    return acc
    
  }, {});

  let resultPerSpecies = Object.entries(probabilityPerSpecies);
  resultPerSpecies.sort(([_, p1], [__, p2]) => p2 - p1);
  resultPerSpecies = resultPerSpecies.filter(([_, probability], i) => probability > 0.01 || i < 20);

  return (
    <div>
      <div className="type-phrase-selection">
        <select value={type} onChange={handleTypeChange}>
          <option value="">Unknown type</option>
          <option value="Typeless">No type</option>
          {types.map(type =>
              <option key={type} value={type}>{type}</option>
          )}

          </select>
        <select value={phrase} onChange={handlePhraseChange}>
          <option value="">Unknown phrase</option>
          {phrases.map(phrase =>
              <option key={phrase} value={phrase}>{phrase}</option>
          )}

          </select>
      </div>
      {result === null ? <p>Loading Pokemon data...</p> : result.length === 0 ? <p>No matching Pokemon found!</p> : 
      <div className="table-container">
      <table className="table">
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
            { result
              .filter(({probability}, i) => probability > 0.01 || i < 20)
              .map(res =>
            <tr key={res.pokemon.species + "-" + res.pokemon.id}>
                <th scope="row">{res.pokemon.species + "-" + res.pokemon.id}</th>
                <td>{(res.probability * 100).toFixed(2) + "%"}</td>
            </tr>
            
            ) }
        </tbody>
        </table>      
        <table className="table">
        <caption>
            Result per species
        </caption>
        <thead>
            <tr>
            <th scope="col">Species</th>
            <th scope="col">Probability</th>
            </tr>
        </thead>
        <tbody>
            { resultPerSpecies
                .map(([species, probability]) => 
            <tr key={species}>
                <th scope="row">{species}</th>
                <td>{(probability * 100).toFixed(2) + "%"}</td>
            </tr>
            
            ) }
        </tbody>
        </table>
        </div>
        }
    </div>
  );
};

export default Calculator;
