import { drawState } from "./game.js";
import { drawUi } from "./ui.js";
import * as wasm from "../wasm/oxidized_turret_bg.js";

// expose JavaScript functions to WASM imports
const importObject = {
  "./oxidized_turret_bg.js": wasm,
};
await WebAssembly.instantiateStreaming(
  fetch("/wasm/oxidized_turret_bg.wasm"),
  importObject
).then((obj) => {
  // expose WASM exports to JavaScript bindings
  wasm.__wbg_set_wasm(obj.instance.exports);
  window.wasm = wasm;
});

// disable browser zoom with keyboard and mouse
window.addEventListener("keydown", function disableKeyboardZoom(event) {
  if (event.ctrlKey === true && ["+", "-", "0"].includes(event.key)) {
    console.log(event.key);
    event.preventDefault();
  }
});

window.addEventListener(
  "wheel",
  function disableMouseZoom(event) {
    if (event.ctrlKey === true) {
      event.preventDefault();
      return false;
    }
  },
  { passive: false }
);

const game = wasm.Game.new();

// draw UI only once (it is redrawn on resize)
drawUi();

function loop(time) {
  game.update_state();
  drawState(game.get_state(), time);
  requestAnimationFrame(loop);
}

requestAnimationFrame(loop);
