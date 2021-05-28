// name:    app.js
// author:  Jos Feenstra
// purpose: entry point 
// note:    I dislike js very much, but I think js is appreciated over a typescript implementation :)

import { setupFileSelectors } from './js/parsing.js';
import init, { } from '../pkg/cityjson_validator.js';

async function main() {
    setupFileSelectors();
}

main();



//#endregion