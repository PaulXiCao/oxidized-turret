import { ExternalState, GameResult } from "../wasm/oxidized_turret_bg.js";
import { Canvas } from "./Canvas.js";
import { Art } from "./Art.js";
import { clamp } from "./utils.js";

export function createGameCanvas(htmlCanvas, initialCanvasState = {}) {
  const canvas = new Canvas(htmlCanvas, {
    scale: 1.0,
    offsetX: 60,
    offsetY: 10,
    ...initialCanvasState,
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
    drawState(state, uiState) {
      if (uiState.selectedTower) {
        const tower = uiState.selectedTower.data.turret;
        canvas.fillCircle(
          tower.pos.x + state.cell_length / 2,
          tower.pos.y + state.cell_length / 2,
          tower.range,
          "rgba(0,255,0,0.1)"
        );

        if (uiState.upgrading) {
          const nextRange = uiState.selectedTower.data.next_stats.find(
            (stat) => stat.key === "Range"
          )?.value;
          if (nextRange) {
            canvas.fillCircle(
              tower.pos.x + state.cell_length / 2,
              tower.pos.y + state.cell_length / 2,
              nextRange * state.cell_length,
              "rgba(0,255,0,0.1)"
            );
          }
        }
      }
    },
    handleWheel({ dirY }) {
      canvas.setScale(clamp(canvas.getScale() + 0.02 * dirY, 0.25, 4));
    },
    handleResize({ width, height }) {
      canvas.resize(width, height);
    },
    handleDragStart(pos) {
      startOffset = canvas.getOffset();
    },
    handleDragMove({ initialPos, currentPos }) {
      canvas.setOffset(
        startOffset.x + currentPos.x - initialPos.x,
        startOffset.y + currentPos.y - initialPos.y
      );
    },
    handleDragEnd(pos) {
      startOffset = null;
    },
    getArt() {
      return gameArt;
    },
    getState() {
      return {
        canvas: canvas.getState(),
      };
    },
  };
}
