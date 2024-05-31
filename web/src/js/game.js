import {
  fillCircle,
  fillRect,
  fillTriangle,
  strokeRect,
  drawLine,
  clear,
} from "./canvas.js";
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

const game = wasm.Game.new();
const state = game.get_state();
const gameWidth = state.board_dimension_x;
const gameHeight = state.board_dimension_y;

const TURRET_SIZE = 30;
const PARTICLE_SIZE = 5;
const CREEP_SIZE = 20;
const HEALTH_BAR_HEIGHT = 2;

function drawTurret(turret) {
  strokeRect({
    x: turret.x,
    y: turret.y,
    width: TURRET_SIZE,
    height: TURRET_SIZE,
  });

  drawLine({
    start: { x: turret.x + TURRET_SIZE / 2, y: turret.y + TURRET_SIZE / 2 },
    end: {
      x:
        turret.x +
        TURRET_SIZE / 2 +
        (TURRET_SIZE / 2) * Math.cos(turret.rotation),
      y:
        turret.y +
        TURRET_SIZE / 2 +
        (TURRET_SIZE / 2) * Math.sin(turret.rotation),
    },
    color: "white",
  });
}

function drawParticle(particle) {
  fillCircle({
    x: particle.x,
    y: particle.y,
    r: PARTICLE_SIZE,
    color: "silver",
  });
}

function drawCreep(creep) {
  fillTriangle({
    x: creep.pos.x,
    y: creep.pos.y,
    size: CREEP_SIZE,
    color: "yellow",
  });

  const healthPercentage = creep.health / creep.max_health;

  fillRect({
    x: creep.pos.x - CREEP_SIZE / 2,
    y: creep.pos.y - CREEP_SIZE / 2 - HEALTH_BAR_HEIGHT,
    width: CREEP_SIZE * healthPercentage,
    height: HEALTH_BAR_HEIGHT,
    color: "green",
  });
}

function drawMap() {
  strokeRect({ x: 0, y: 0, width: gameWidth, height: gameHeight });
}

/**
 *
 * @param {wasm.State} state
 */
function drawState(state) {
  clear();
  drawMap();

  for (const turret of state.turrets) {
    drawTurret(turret);
  }
  for (const creep of state.creeps) {
    drawCreep(creep);
  }
  for (const particle of state.particles) {
    drawParticle(particle);
  }
}

function loop(time) {
  game.update_state();
  drawState(game.get_state());
  requestAnimationFrame(loop);
}

requestAnimationFrame(loop);
