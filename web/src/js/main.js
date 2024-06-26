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
const uiCanvas = document.getElementById("ui-canvas");
const ui = createUi({
  canvas: document.getElementById("ui-canvas"),
  health: document.querySelector(".health"),
  wave: document.querySelector(".wave"),
  gold: document.querySelector(".gold"),
  speed: document.querySelector(".speed"),
  global: document.body,
  result: document.querySelector(".result"),
  towerDetailSidebar: document.querySelector(".tower-detail-sidebar"),
  towerStats: document.querySelector(".tower-stats"),
  towerUpgradeButton: document.querySelector(".tower-upgrade"),
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
  } else if (event.target.classList.contains("turret")) {
    console.log(event.target.dataset.type);
  } else if (event.target.classList.contains("tower-upgrade")) {
    stateHandler.handleTowerUpgrade();
  } else if (event.target.classList.contains("tower-sell")) {
    stateHandler.handleTowerSell();
  } else if (event.target.classList.contains("close")) {
    stateHandler.handleSidebarClose();
  }
});

let lastPointerDown = null;
let isDragging = false;

uiCanvas.addEventListener("pointerdown", function mainMousedownHandler(event) {
  lastPointerDown = { x: event.clientX, y: event.clientY };
  isDragging = false;
});

uiCanvas.addEventListener("pointerup", function mainMouseupHandler(event) {
  const pos = { x: event.clientX, y: event.clientY };
  if (
    lastPointerDown &&
    Math.abs(lastPointerDown.x - event.clientX) +
      Math.abs(lastPointerDown.y - event.clientY) <
      2
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
uiCanvas.addEventListener("pointermove", function currentMousePosition(event) {
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
