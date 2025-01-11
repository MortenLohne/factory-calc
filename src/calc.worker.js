/* eslint-disable no-restricted-globals */

import init, { Data, PokemonData, KnownPokemon } from 'rust-calc';

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
        // const knownPokemon = payload.includedMons.filter(mon => mon.species).map((mon) => (new KnownPokemon(mon.species, mon.possibleMonIds)));

        const knownPokemon = payload.opponentMovesets.filter(mons => mons.length > 0).map((mons) => new KnownPokemon(mons[0].species, mons.map((mon) => mon.id)));
        console.log("Known mons: ", knownPokemon.map(mon => mon.toString()))

        if (payload.useHighAccuracy) {
            const firstMon = knownPokemon[0];
            const backMons = knownPokemon.slice(1);
            console.log(`First mon ${firstMon?.species}-${firstMon?.possibleSets} and back mons ${backMons}`)
            probabilities = pokemonData.compute_wasm(payload.type, payload.phrase, firstMon, backMons, payload.excludedSpecies);
        } else {
            probabilities = data.compute(payload.type, payload.phrase, knownPokemon, payload.excludedSpecies);
        }
        const endTime = performance.now()
        console.log(`Computed probabilities in ${endTime - startTime} milliseconds`)
        postMessage(JSON.stringify(
            { pokemonProbabilities: probabilities }))
    };
});