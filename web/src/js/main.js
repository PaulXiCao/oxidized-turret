import { game } from "./game.js";
import { ui } from "./ui.js";
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

const gameEngine = wasm.Game.new();

let lastPointerDown = null;
window.addEventListener("pointerdown", function mainMousedownHandler(event) {
  lastPointerDown = { x: event.clientX, y: event.clientY };
  game.handleMousedown(event);
});

window.addEventListener("pointerup", function mainMouseupHandler(event) {
  if (
    lastPointerDown &&
    lastPointerDown.x === event.clientX &&
    lastPointerDown.y === event.clientY
  ) {
    ui.handleClick(event);
  }
  game.handleMouseup(event);
});

let mouseX = 0;
let mouseY = 0;
window.addEventListener("pointermove", function currentMousePosition(event) {
  mouseX = event.clientX;
  mouseY = event.clientY;
});

// custom events
window.addEventListener("buildTower", function buildTowerEventListener(event) {
  const canvasPos = game.realToCanvas(event.detail.screenPos);
  gameEngine.build_tower(canvasPos.x, canvasPos.y);
});

// draw UI only once (it is internally redrawn when needed)
ui.drawUi();

function loop(time) {
  gameEngine.update_state();
  const gameState = gameEngine.get_state();
  game.drawState(gameState, time);

  const uiState = ui.getState();
  if (uiState.selectedTurret === 0 && mouseX > 50) {
    game.indicateTurret(gameState, { x: mouseX, y: mouseY });
  }

  requestAnimationFrame(loop);
}

requestAnimationFrame(loop);
