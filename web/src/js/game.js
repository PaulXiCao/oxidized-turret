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

const gameArt = new Art(canvas);

export const game = {
  realToCanvas(screenPos) {
    return canvas.realToCanvas(screenPos);
  },
  /**
   * @param {ExternalState} state
   */
  drawState(state, time) {
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
  },
  /**
   * @param {ExternalState} state
   */
  indicateTurret(state, realPos) {
    const { x, y } = canvas.realToCanvas(realPos);

    let gridX = Math.floor(x / state.cell_length) * state.cell_length;
    let gridY = Math.floor(y / state.cell_length) * state.cell_length;

    canvas.fillRect({
      x: gridX,
      y: gridY,
      width: state.cell_length,
      height: state.cell_length,
      color: "rgba(255, 255, 255, 0.1)",
    });
    gameArt.drawTurret(
      { pos: { x: gridX, y: gridY }, rotation: 0 },
      state.cell_length
    );
  },
  /**
   * @param {MouseEvent} event
   */
  handleMousedown(event) {
    startMove.x = event.clientX;
    startMove.y = event.clientY;
    startOffset = canvas.getOffset();

    window.addEventListener("mousemove", mousemove);
  },
  /**
   * @param {MouseEvent} event
   */
  handleMouseup(event) {
    window.removeEventListener("mousemove", mousemove);
  },
};
