/* eslint-disable no-restricted-globals */

import init, { Data, PokemonData } from 'rust-calc';

init().then(() => {
    const startTime = performance.now()
    const pokemonData = new PokemonData();
    postMessage(JSON.stringify({ pokemonData: pokemonData.allPokemon() }));
    const data = new Data();
    const endTime = performance.now()
    console.log(`Loaded pokemon data in ${endTime - startTime} milliseconds`)

    const defaultResult = data.compute("Typeless", "FreeSpirited", [], []);
    postMessage(JSON.stringify({ pokemonProbabilities: defaultResult }))

    self.onmessage = function (e) {
        const payload = JSON.parse(e.data)
        console.log(`Computing with "${payload.type}", "${payload.phrase}", highAccuracy=${payload.useHighAccuracy}, included mons ${JSON.stringify(payload.includedMons)} and excluded species "${payload.excludedSpecies}" from "${payload}`)
        const startTime = performance.now()
        let probabilities;
        const includedSpecies = payload.includedMons.map(mon => mon.species).filter(species => species !== "");
        if (payload.useHighAccuracy) {
            const firstMon = includedSpecies[0];
            const backMons = includedSpecies.slice(1);
            console.log(`First mon ${firstMon} and back mons ${backMons}`)
            probabilities = pokemonData.compute_wasm(payload.type, payload.phrase, firstMon, backMons, payload.excludedSpecies);
        } else {
            probabilities = data.compute(payload.type, payload.phrase, includedSpecies, payload.excludedSpecies);
        }
        const endTime = performance.now()
        console.log(`Computed probabilities in ${endTime - startTime} milliseconds`)
        postMessage(JSON.stringify(
            { pokemonProbabilities: probabilities }))
    };
});