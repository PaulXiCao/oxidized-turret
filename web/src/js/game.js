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

const canvas = document.getElementById("canvas");
canvas.width = window.innerWidth;
canvas.height = window.innerHeight;

window.addEventListener("resize", function () {
  canvas.width = window.innerWidth;
  canvas.height = window.innerHeight;
});

let scale = 1.0;
let offsetX = 300;
let offsetY = 150;

window.addEventListener("wheel", (event) => {
  scale = Math.min(Math.max(scale + 0.02 * Math.sign(event.deltaY), 0.25), 2);
});

let startMoveX = 0;
let startMoveY = 0;
let startOffsetX = 0;
let startOffsetY = 0;

function mousemove(event) {
  offsetX = startOffsetX + event.clientX - startMoveX;
  offsetY = startOffsetY + event.clientY - startMoveY;
}

window.addEventListener("mousedown", (event) => {
  startMoveX = event.clientX;
  startMoveY = event.clientY;
  startOffsetX = offsetX;
  startOffsetY = offsetY;
  window.addEventListener("mousemove", mousemove);
});

window.addEventListener("mouseup", (event) => {
  window.removeEventListener("mousemove", mousemove);
});

const TURRET_SIZE = 30;
const PARTICLE_SIZE = 5;
const CREEP_SIZE = 20;
const HEALTH_BAR_HEIGHT = 2;

/** @type CanvasRenderingContext2D */
const ctx = canvas.getContext("2d");

function strokeRect({ x, y, width, height, color }) {
  ctx.strokeStyle = color;
  ctx.strokeRect(
    x / scale + offsetX,
    y / scale + offsetY,
    width / scale,
    height / scale
  );
}

function fillRect({ x, y, width, height, color }) {
  ctx.fillStyle = color;
  ctx.fillRect(
    x / scale + offsetX,
    y / scale + offsetY,
    width / scale,
    height / scale
  );
}

function drawLine({ start, end, color }) {
  ctx.strokeStyle = color;
  ctx.beginPath();
  ctx.moveTo(start.x / scale + offsetX, start.y / scale + offsetY);
  ctx.lineTo(end.x / scale + offsetX, end.y / scale + offsetY);
  ctx.stroke();
}

function fillCircle({ x, y, r, color }) {
  ctx.fillStyle = color;
  ctx.beginPath();
  ctx.arc(x / scale + offsetX, y / scale + offsetY, r / scale, 0, 2 * Math.PI);
  ctx.fill();
}

function fillTriangle({ x, y, size, color }) {
  ctx.fillStyle = color;
  ctx.beginPath();
  ctx.moveTo(
    (x - size / 2) / scale + offsetX,
    (y + size / 2) / scale + offsetY
  );
  ctx.lineTo(x / scale + offsetX, (y - size / 2) / scale + offsetY);
  ctx.lineTo(
    (x + size / 2) / scale + offsetX,
    (y + size / 2) / scale + offsetY
  );
  ctx.lineTo(
    (x - size / 2) / scale + offsetX,
    (y + size / 2) / scale + offsetY
  );
  ctx.closePath();
  ctx.fill();
}

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
  if (!particle.visible) {
    return;
  }
  fillCircle({
    x: particle.x,
    y: particle.y,
    r: PARTICLE_SIZE,
    color: "silver",
  });
}
function drawCreep(creep) {
  fillTriangle({ x: creep.x, y: creep.y, size: CREEP_SIZE, color: "yellow" });

  const healthPercentage = creep.health / creep.maxHealth;

  fillRect({
    x: creep.x - CREEP_SIZE / 2,
    y: creep.y - CREEP_SIZE / 2 - HEALTH_BAR_HEIGHT,
    width: CREEP_SIZE * healthPercentage,
    height: HEALTH_BAR_HEIGHT,
    color: "green",
  });
}

/**
 *
 * @param {wasm.State} state
 */
function drawState(state) {
  ctx.clearRect(0, 0, canvas.clientWidth, canvas.clientHeight);

  ctx.strokeStyle = "white";
  strokeRect({ x: 0, y: 0, width: gameWidth, height: gameHeight });

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
