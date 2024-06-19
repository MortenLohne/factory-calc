import React, { useState } from 'react';
import './App.css';
// import FibonacciCalculator from './FibonacciCalculator.js';
import Calculator from './Calculator.jsx';

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <h1> Battle Factory Calculator</h1>
      </header>
      <div className="App-body">
      <Calculator/>
      </div>
    </div>
  );
}

export default App;
