import { readFile } from "node:fs/promises";
import * as wasm from "../src/wasm/oxidized_turret_bg.js";

const memory = new WebAssembly.Memory({
  initial: 17,
  maximum: 10000,
});

// expose JavaScript functions to WASM imports
const importObject = {
  "./oxidized_turret_bg.js": wasm,
  env: { memory },
};

await WebAssembly.instantiate(
  await readFile("./src/wasm/oxidized_turret_bg.wasm"),
  importObject
).then((wasmModule) => {
  // expose WASM exports to JavaScript bindings
  wasm.__wbg_set_wasm(wasmModule.instance.exports);
});

export function createNewGame() {
  return wasm.Game.new();
}
