/* eslint-disable no-restricted-globals */

import init, { Data } from 'rust-calc';

init().then(() => {
    const startTime = performance.now()
    const data = new Data();
    const endTime = performance.now()
    console.log(`Loaded pokemon data in ${endTime - startTime} milliseconds`)

    const defaultResult = data.compute("Typeless", "FreeSpirited", [], []);
    postMessage(JSON.stringify(defaultResult))

    self.onmessage = function (e) {
        const payload = JSON.parse(e.data)
        // console.log(`Computing with "${payload.type}", "${payload.phrase}" included species ${payload.includedSpecies} and excluded species "${payload.excludedSpecies}" from "${payload}`)
        const startTime = performance.now()
        const probabilities = data.compute(payload.type, payload.phrase, payload.includedSpecies, payload.excludedSpecies);
        const endTime = performance.now()
        console.log(`Computed probabilities in ${endTime - startTime} milliseconds`)
        postMessage(JSON.stringify(
            probabilities))
    };
});