/**
 * Main glue code for the JS Frontend.
 * All browser global interactions (document, window, event listeners)
 * should happen here.
 */
import { createGameCanvas } from "./game.js";
import { createUi } from "./ui.js";
import * as wasm from "../wasm/oxidized_turret_bg.js";
import { createStateHandler } from "./state.js";

// expose JavaScript functions to WASM imports
const importObject = {
  "./oxidized_turret_bg.js": wasm,
};
await WebAssembly.instantiateStreaming(
  fetch("./wasm/oxidized_turret_bg.wasm"),
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
const ui = createUi({
  canvas: document.getElementById("ui-canvas"),
  health: document.querySelector(".health"),
  wave: document.querySelector(".wave"),
  gold: document.querySelector(".gold"),
  speed: document.querySelector(".speed"),
  global: document.body,
  result: document.querySelector(".result"),
});

const gameCanvas = createGameCanvas(document.getElementById("canvas"));
const stateHandler = createStateHandler({ gameEngine, gameCanvas, ui });

document.addEventListener("click", (event) => {
  if (event.target.classList.contains("increase-speed")) {
    stateHandler.increaseAnimationSpeed();
  } else if (event.target.classList.contains("decrease-speed")) {
    stateHandler.decreaseAnimationSpeed();
  } else if (event.target.classList.contains("start")) {
    stateHandler.handleStartButton();
  }
});

let lastPointerDown = null;
let isDragging = false;

window.addEventListener("pointerdown", function mainMousedownHandler(event) {
  lastPointerDown = { x: event.clientX, y: event.clientY };
  isDragging = false;
});

window.addEventListener("pointerup", function mainMouseupHandler(event) {
  const pos = { x: event.clientX, y: event.clientY };
  if (
    lastPointerDown &&
    lastPointerDown.x === event.clientX &&
    lastPointerDown.y === event.clientY
  ) {
    stateHandler.handleClick(pos);
  } else if (lastPointerDown) {
    stateHandler.handleDragEnd(pos);
    isDragging = false;
  }

  lastPointerDown = null;
});

let mouseX = 0;
let mouseY = 0;
window.addEventListener("pointermove", function currentMousePosition(event) {
  mouseX = event.clientX;
  mouseY = event.clientY;

  const pos = { x: event.clientX, y: event.clientY };
  if (lastPointerDown) {
    if (!isDragging) {
      stateHandler.handleDragStart(pos);
      isDragging = true;
    }
    stateHandler.handleDragMove({
      initialPos: lastPointerDown,
      currentPos: pos,
    });
  }
});

window.addEventListener("keyup", function shortcutHandler(event) {
  stateHandler.handleKeyUp({ key: event.key });
});

window.addEventListener("resize", function () {
  stateHandler.handleResize({
    width: window.innerWidth,
    height: window.innerHeight,
  });
});
// trigger resize event once on load
stateHandler.handleResize({
  width: window.innerWidth,
  height: window.innerHeight,
});

window.addEventListener("wheel", (event) => {
  stateHandler.handleWheel({ dirY: Math.sign(event.deltaY) });
});

function loop(time) {
  stateHandler.handleTimeStep(time);
  requestAnimationFrame(loop);
}

requestAnimationFrame(loop);
