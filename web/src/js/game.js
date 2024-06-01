import {
  fillCircle,
  fillRect,
  fillTriangle,
  strokeRect,
  drawLine,
  drawPath,
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

const TURRET_SIZE = 30;
const PARTICLE_SIZE = 5;
const CREEP_SIZE = 20;
const HEALTH_BAR_HEIGHT = 2;

function drawTurret(turret, cellLength) {
  const x = turret.pos.x * cellLength;
  const y = turret.pos.y * cellLength;
  strokeRect({
    x,
    y,
    width: TURRET_SIZE,
    height: TURRET_SIZE,
  });

  drawLine({
    start: {
      x: x + TURRET_SIZE / 2,
      y: y + TURRET_SIZE / 2,
    },
    end: {
      x: x + TURRET_SIZE / 2 + (TURRET_SIZE / 2) * Math.cos(turret.rotation),
      y: y + TURRET_SIZE / 2 + (TURRET_SIZE / 2) * Math.sin(turret.rotation),
    },
    color: "white",
  });
}

function drawParticle(particle) {
  fillCircle({
    x: particle.pos.x,
    y: particle.pos.y,
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

/**
 * @param {wasm.ExternalState} state
 */
function drawMap(state, time) {
  strokeRect({
    x: 0,
    y: 0,
    width: state.board_dimension_x * state.cell_length,
    height: state.board_dimension_y * state.cell_length,
    color: "white",
  });

  const points = state.creep_path.map(({ x, y }) => {
    return {
      x: (x + 0.5) * state.cell_length,
      y: y * state.cell_length + state.cell_length / 2,
    };
  });

  drawPath({
    points,
    color: "white",
    segments: [3, 5],
    dashOffset: -time / 60,
  });

  fillRect({
    x: state.creep_spawn.x * state.cell_length,
    y: state.creep_spawn.y * state.cell_length,
    width: state.cell_length,
    height: state.cell_length,
    color: "green",
  });

  fillRect({
    x: state.creep_goal.x * state.cell_length,
    y: state.creep_goal.y * state.cell_length,
    width: state.cell_length,
    height: state.cell_length,
    color: "red",
  });
}

/**
 * @param {wasm.ExternalState} state
 */
function drawState(state, time) {
  clear();
  drawMap(state, time);

  for (const turret of state.turrets) {
    drawTurret(turret, state.cell_length);
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
  drawState(game.get_state(), time);
  requestAnimationFrame(loop);
}

requestAnimationFrame(loop);
