import { ExternalState } from "../wasm/oxidized_turret_bg.js";
import { Canvas } from "./Canvas.js";
import { Art } from "./Art.js";
import { clamp } from "./utils.js";

const canvas = new Canvas(document.getElementById("canvas"), {
  scale: 1.0,
  offsetX: 60,
  offsetY: 10,
});
canvas.resize({ width: window.innerWidth, height: window.innerHeight });

window.addEventListener("resize", function () {
  canvas.resize({ width: window.innerWidth, height: window.innerHeight });
});

window.addEventListener("wheel", (event) => {
  canvas.setScale(
    clamp(canvas.getScale() + 0.02 * Math.sign(event.deltaY), 0.25, 4)
  );
});

let startMove = { x: 0, y: 0 };
let startOffset = { x: 0, y: 0 };

function mousemove(event) {
  canvas.setOffset({
    x: startOffset.x + event.clientX - startMove.x,
    y: startOffset.y + event.clientY - startMove.y,
  });
}

window.addEventListener("mousedown", (event) => {
  startMove.x = event.clientX;
  startMove.y = event.clientY;
  startOffset = canvas.getOffset();

  window.addEventListener("mousemove", mousemove);
});

window.addEventListener("mouseup", (event) => {
  window.removeEventListener("mousemove", mousemove);
});

const gameArt = new Art(canvas);

/**
 * @param {ExternalState} state
 */
export function drawState(state, time) {
  gameArt.clear();
  gameArt.drawMap(state, time);

  for (const turret of state.turrets) {
    gameArt.drawTurret(turret, state.cell_length);
  }
  for (const creep of state.creeps) {
    gameArt.drawCreep(creep);
  }
  for (const particle of state.particles) {
    gameArt.drawParticle(particle);
  }
}
