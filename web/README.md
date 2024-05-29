# Oxidized Turret Web Frontend

You need to install the Node.js >= 20.
Then, run `node server.mjs` in this folder.
Afterwards, the game should run on http://localhost:8080/index.html.

## ToDo

### Missing npm packages?

When adding one of the following lines into `src/game.js` the website silently breaks (no commandline error message and website only shows heading)
```js
import * as OT from "./wasm_pack_output/oxidized_turret.js";
import * as wasm from "./oxidized_turret_bg.wasm";
```
According to this documentation section we need the following npm packages: webpack, parcel, rollup.
Should we create a package.json?
