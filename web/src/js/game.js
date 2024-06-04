import { ExternalState, GameResult } from "../wasm/oxidized_turret_bg.js";
import { Canvas } from "./Canvas.js";
import { Art } from "./Art.js";
import { clamp } from "./utils.js";

export function createGameCanvas(htmlCanvas) {
  const canvas = new Canvas(htmlCanvas, {
    scale: 1.0,
    offsetX: 60,
    offsetY: 10,
  });

  let startOffset = null;

  const gameArt = new Art(canvas);

  return {
    realToCanvas(screenPos) {
      return canvas.realToCanvas(screenPos);
    },
    /**
     * @param {ExternalState} state
     */
    drawState(state, time) {
      switch (state.game_result) {
        case GameResult.PlayerWon:
          alert("player won!!");
        case GameResult.CreepsWon:
          alert("creeps won!!");
        default:;
      }
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
    handleWheel({ dirY }) {
      canvas.setScale(clamp(canvas.getScale() + 0.02 * dirY, 0.25, 4));
    },
    handleResize({ width, height }) {
      canvas.resize({ width, height });
    },
    handleDragStart(pos) {
      startOffset = canvas.getOffset();
    },
    handleDragMove({ initialPos, currentPos }) {
      canvas.setOffset({
        x: startOffset.x + currentPos.x - initialPos.x,
        y: startOffset.y + currentPos.y - initialPos.y,
      });
    },
    handleDragEnd(pos) {
      startOffset = null;
    },
  };
}
