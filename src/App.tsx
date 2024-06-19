import React, { useState } from 'react';
import logo from './logo.svg';
import './App.css';
// import FibonacciCalculator from './FibonacciCalculator.js';
import Calculator from './Calculator.js';

function App() {
  const [greeting, setGreeting] = useState('');
  const [pokemonList, setPokemonList] = useState({});


  const handleGreet = () => {

    setGreeting('React-Rust Developer');

  };

  return (
    <div className="App">
      <header className="App-header">
        <Calculator />
      </header>
    </div>
  );
}

export default App;
