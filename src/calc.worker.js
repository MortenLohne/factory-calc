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
        // console.log(`Computing with "${payload.type}", "${payload.phrase}" and "${payload.excludedSpecies}" from "${payload}`)
        const probabilities = data.compute(payload.type, payload.phrase, [], payload.excludedSpecies);
        postMessage(JSON.stringify(
            probabilities))
    };
});