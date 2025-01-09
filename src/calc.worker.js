/* eslint-disable no-restricted-globals */

import init, { Data, PokemonData } from 'rust-calc';

init().then(() => {
    const startTime = performance.now()
    const data = new Data();
    const pokemonData = new PokemonData();
    const endTime = performance.now()
    console.log(`Loaded pokemon data in ${endTime - startTime} milliseconds`)

    const defaultResult = data.compute("Typeless", "FreeSpirited", [], []);
    postMessage(JSON.stringify(defaultResult))

    self.onmessage = function (e) {
        const payload = JSON.parse(e.data)
        console.log(`Computing with "${payload.type}", "${payload.phrase}", highAccuracy=${payload.useHighAccuracy}, included species ${payload.includedSpecies} and excluded species "${payload.excludedSpecies}" from "${payload}`)
        const startTime = performance.now()
        let probabilities;
        if (payload.useHighAccuracy) {
            const firstMon = payload.includedSpecies[0];
            const backMons = payload.includedSpecies.slice(1);
            probabilities = pokemonData.compute_wasm(payload.type, payload.phrase, firstMon, backMons, payload.excludedSpecies);
        } else {
            probabilities = data.compute(payload.type, payload.phrase, payload.includedSpecies, payload.excludedSpecies);
        }
        const endTime = performance.now()
        console.log(`Computed probabilities in ${endTime - startTime} milliseconds`)
        postMessage(JSON.stringify(
            probabilities))
    };
});