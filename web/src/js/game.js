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
    drawState(state, uiState, time) {
      gameArt.clear();

      gameArt.drawMap(state.board_dimension_x, state.board_dimension_y);
      gameArt.drawPath(state.creep_path, time);
      gameArt.drawCreepSpawn(
        state.creep_spawn.x,
        state.creep_spawn.y,
        state.cell_length
      );
      for (const goal of state.creep_goals) {
        gameArt.drawCreepGoal(goal.x, goal.y, state.cell_length);
      }

      for (const turret of state.turrets) {
        gameArt.drawTurret(
          turret.pos.x,
          turret.pos.y,
          turret.rotation,
          state.cell_length,
          turret.kind
        );
      }
      for (const creep of state.creeps) {
        gameArt.drawCreep(
          creep.pos.x,
          creep.pos.y,
          creep.health / creep.max_health,
          state.current_level
        );
      }
      for (const particle of state.particles) {
        gameArt.drawParticle(particle.pos.x, particle.pos.y);
      }

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
  };
}
