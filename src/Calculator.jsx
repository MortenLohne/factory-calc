// src/FibonacciCalculator.js
import React, { useState, useEffect } from 'react';
import './Calculator.css'; // Assuming you have a CSS file for styles
import Worker from './calc.worker.js';
import ExcludedSpeciesSelector from './ExcludedSpeciesSelector';
import OpponentSelector from './OpponentSelector';

const types = [
  'Bug', 'Dark',
  'Dragon', 'Electric',
  'Fighting', 'Fire',
  'Flying', 'Ghost',
  'Grass', 'Ground',
  'Ice', 'Normal',
  'Poison', 'Psychic',
  'Rock', 'Steel',
  'Water'
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

const phraseLabels = {
  "FreeSpirited": "Free-spirited and unrestrained",
  "Preparation": "Total preparation",
  "SlowAndSteady": "Slow and steady",
  "Endurance": "One of endurance",
  "HighRiskHighReturn": "High risk, high return",
  "Weakening": "Weakening the foe",
  "Unpredictable": "Impossible to predict",
  "BattleFlow": "Depends on the battle's flow",
  "Adaptable": "Flexibly adaptable",
}


const Calculator = () => {
  const [type, setType] = useState("Typeless");
  const [phrase, setPhrase] = useState("FreeSpirited");
  const [includedSpecies, setIncludedSpecies] = useState([]); // Species on the opponent's team
  const [excludedSpecies, setExcludedSpecies] = useState([]); // Species on our team or in last round's pool
  const [result, setResult] = useState(null);
  const [worker] = useState(() => new Worker());

  const handleTypeChange = (e) => {
    setType(e.target.value);
  };

  const handlePhraseChange = (e) => {
    setPhrase(e.target.value);
  };

  useEffect(() => {
    worker.postMessage(JSON.stringify({ type: type, phrase: phrase, includedSpecies, excludedSpecies: excludedSpecies }));
  }, [type, phrase, includedSpecies, excludedSpecies, worker]);

  worker.onmessage = (e) => {
    setResult(JSON.parse(e.data));
  };

  const probabilityPerSpecies = (result || []).reduce((acc, { pokemon, probability }) => {
    acc[pokemon.species] = (acc[pokemon.species] || 0) + probability
    return acc

  }, {});

  let resultPerSpecies = Object.entries(probabilityPerSpecies);
  resultPerSpecies.sort(([_, p1], [__, p2]) => p2 - p1);
  resultPerSpecies = resultPerSpecies.filter(([_, probability], i) => probability > 0.0 && (probability > 0.01 || i < 20));

  return (
    <div>
      <div className="type-phrase-selection">
        <select value={type} onChange={handleTypeChange} style={{ fontSize: '1.3em' }}>
          <option value="">Unknown type</option>
          <option value="Typeless">No type</option>
          {types.map(type =>
            <option key={type} value={type}>{type}</option>
          )}
        </select>
        <select value={phrase} onChange={handlePhraseChange} style={{ fontSize: '1.3em' }}>
          <option value="">Unknown phrase</option>
          {phrases.map(phrase =>
            <option key={phrase} value={phrase}>{phraseLabels[phrase]}</option>
          )}
        </select>
      </div>
      <h2 style={{ marginBottom: 0, marginTop: 30 }}> Excluded pokémon: </h2>
      <span style={{ fontSize: 12 }}> (Our team, the last opponent's team, or Pokémon rejected from the draft) </span>
      <div className="excluded-species">
        <ExcludedSpeciesSelector setExcludedSpecies={setExcludedSpecies} />

      </div>
      <h2 style={{ marginBottom: 0, marginTop: 50 }}> Opponent's pokémon: </h2>
      <div>
        <OpponentSelector setIncludedSpecies={setIncludedSpecies} />
      </div>
      {result === null ? <p>Loading Pokemon data...</p> : result.length === 0 ? <p>No matching Pokemon found!</p> :
        <div className="table-container">
          <table className="table">
            <caption>
              Result
            </caption>
            <thead>
              <tr>
                <th scope="col">Species</th>
                <th scope="col">Probability</th>
              </tr>
            </thead>
            <tbody>
              {resultPerSpecies
                .map(([species, probability]) =>
                  <tr key={species}>
                    <th scope="row">{species}</th>
                    <td>{(probability * 100).toFixed(2) + "%"}</td>
                  </tr>
                )}
            </tbody>
          </table>
          <table className="table">
            <caption>
              Result per moveset
            </caption>
            <thead>
              <tr>
                <th scope="col">Pokemon</th>
                <th scope="col">Probability</th>
              </tr>
            </thead>
            <tbody>
              {result
                .filter(({ probability }, i) => probability > 0.0 && (probability > 0.01 || i < 20))
                .map(res =>
                  <tr key={res.pokemon.species + "-" + res.pokemon.id}>
                    <th scope="row">{res.pokemon.species + "-" + res.pokemon.id}</th>
                    <td>{(res.probability * 100).toFixed(2) + "%"}</td>
                  </tr>
                )}
            </tbody>
          </table>
        </div>
      }
    </div>
  );
};

export default Calculator;
