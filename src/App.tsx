import React, { useState } from 'react';
import logo from './logo.svg';
import './App.css';
import { greet } from './pkg/rust_calc';
import { Data } from './pkg/rust_calc';
// import FibonacciCalculator from './FibonacciCalculator.js';
import Calculator from './Calculator.js';

function App() {
  const [greeting, setGreeting] = useState('');
  const [pokemonList, setPokemonList] = useState({});


  const handleGreet = () => {

    const message = greet('React-Rust Developer');

    setGreeting('React-Rust Developer');
    /* eslint-disable no-restricted-globals */
    const data: any = new (Data as any)();
    const probabilities = data.compute("Normal", "Preparation", []);

    setGreeting(probabilities[0].pokemon.toString() + ": " + probabilities[0].probability)

  };

  return (
    <div className="App">
      <header className="App-header">
        <button onClick={handleGreet}>Greet</button>

        <p> Hello world!</p>
        <p>{greeting}</p>
        <Calculator />
      </header>
    </div>
  );
}

export default App;
