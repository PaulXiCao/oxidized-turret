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

const canvas = document.getElementById("canvas");

/** @type CanvasRenderingContext2D */
const ctx = canvas.getContext("2d");

function drawTurret(turret) {
  ctx.strokeStyle = "white";
  ctx.strokeRect(turret.x - 15, turret.y - 15, 30, 30);

  ctx.strokeStyle = "white";
  ctx.beginPath();
  ctx.moveTo(turret.x, turret.y);
  ctx.lineTo(
    turret.x + 15 * Math.cos(turret.rotation),
    turret.y + 15 * Math.sin(turret.rotation)
  );
  ctx.stroke();
}
function drawParticle(particle) {
  if (!particle.visible) {
    return;
  }
  ctx.fillStyle = "silver";
  ctx.strokeStyle = "silver";
  ctx.beginPath();
  ctx.arc(particle.x, particle.y, 5, 0, 2 * Math.PI);
  ctx.fill();
}
function drawCreep(creep) {
  ctx.fillStyle = "yellow";
  ctx.beginPath();
  ctx.moveTo(creep.x - 10, creep.y + 10);
  ctx.lineTo(creep.x, creep.y - 10);
  ctx.lineTo(creep.x + 10, creep.y + 10);
  ctx.lineTo(creep.x - 10, creep.y + 10);
  ctx.closePath();
  ctx.fill();

  const healthPercentage = creep.health / creep.maxHealth;

  ctx.fillStyle = "green";
  ctx.fillRect(creep.x - 10, creep.y - 12, 20 * healthPercentage, 2);
}

/**
 *
 * @param {wasm.State} state
 */
function drawState(state) {
  ctx.fillStyle = "black";
  ctx.fillRect(0, 0, canvas.clientWidth, canvas.clientHeight);

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
