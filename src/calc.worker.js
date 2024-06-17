/* eslint-disable no-restricted-globals */

import init, { Data } from './pkg/rust_calc';

init().then(() => {
    const data = new Data();

    self.onmessage = function(e) {
        console.log("Hello from worker!")
        console.log("Initialized data")
        console.log("Got e.data: " + JSON.stringify(e.data) + ", " + e.data)
        const payload = JSON.parse(e.data)
        console.log(`Computing with "${payload.type}" and "${payload.phrase}", from "${payload}`)
        const probabilities = data.compute(payload.type, payload.phrase, []);
        // const probabilities = [{pokemon: "yes", probability: 0.5}];

        postMessage(JSON.stringify(
            probabilities.slice(0, 10).map(prob  => ({ 
                name: prob.pokemon.toString(), 
                p: prob.probability
            })
        )))
    };
});