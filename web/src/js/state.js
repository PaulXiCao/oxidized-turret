/**
 * This file contains the main frontend logic to update and interact
 * with the game screen and the UI.
 * It uses the state pattern (basically a finite-state machine).
 * This decouples the state from the behavior and reduce errors
 *
 * It handles all interactions between the game engine and the UI elements.
 *
 * However, this file should NOT depend on any browser events. This improves testability.
 */

import * as wasm from "../wasm/oxidized_turret_bg.js";

/**
 * @param {object} options
 * @param {wasm.Game} options.gameEngine
 */
export function createStateHandler({
  gameEngine,
  gameCanvas,
  ui,
  art,
  initialUiState = {},
}) {
  let state = {
    phase: wasm.GamePhase.Building,
    result: wasm.GameResult.StillRunning,
    selectedTurret: null,
    selectedTower: undefined,
    upgrading: false,
    health: 20,
    wave: 1,
    gold: 200,
    animationSpeed: 3,
    ...initialUiState,
  };

  if (state.selectedTower) {
    state.selectedTower = wasm.TurretRef.__wrap(state.selectedTower.__wbg_ptr);
  }

  const uiState = new Proxy(state, {
    set(target, propertyKey, value, receiver) {
      const prevValue = Reflect.get(target, propertyKey, receiver);
      if (prevValue === value) {
        return true;
      }

      const result = Reflect.set(target, propertyKey, value, receiver);
      ui.drawUi(uiState);
      return result;
    },
  });

  return {
    handleClick(clickPos) {
      if (clickPos.x <= 50) {
        // We are selecting a turret
        let new_turret = null;
        if (clickPos.y < 50) {
          new_turret = 0;
        } else if (clickPos.y < 100) {
          new_turret = 1;
        } else if (clickPos.y < 150) {
          new_turret = 2;
        } else if (clickPos.y < 200) {
          new_turret = 3;
        }

        if (new_turret !== null) {
          if (uiState.selectedTurret === new_turret) {
            // delesect turret
            uiState.selectedTurret = null;
          } else {
            uiState.selectedTurret = new_turret;
          }
        }
      }

      if (clickPos.x > 50 && uiState.selectedTurret !== null) {
        const canvasPos = gameCanvas.realToCanvas(clickPos);
        gameEngine.build_tower(
          canvasPos.x,
          canvasPos.y,
          uiState.selectedTurret
        );
      }

      if (clickPos.x > 50 && uiState.selectedTurret === null) {
        const canvasPos = gameCanvas.realToCanvas(clickPos);
        const tower = gameEngine.get_tower_at(canvasPos.x, canvasPos.y);
        uiState.selectedTower = tower;
        uiState.upgrading = false;
      }

      return false;
    },
    handleResize({ width, height }) {
      gameCanvas.handleResize({ width, height });
      ui.handleResize({ width, height });
      ui.drawUi(uiState);
    },
    handleDragStart(pos) {
      gameCanvas.handleDragStart(pos);
    },
    handleDragMove({ initialPos, currentPos }) {
      gameCanvas.handleDragMove({ initialPos, currentPos });
    },
    handleDragEnd(pos) {
      gameCanvas.handleDragEnd(pos);
    },
    handleWheel({ dirY }) {
      gameCanvas.handleWheel({ dirY });
    },
    handleKeyUp({ key }) {
      if (key === "1") {
        uiState.selectedTurret = uiState.selectedTurret === null ? 0 : null;
      }
    },
    handleTimeStep(time) {
      for (let i = 0; i < 2 ** uiState.animationSpeed; i += 1) {
        gameEngine.update_state();
      }

      const gameState = gameEngine.get_state();
      uiState.health = gameState.health;
      uiState.gold = gameState.gold;
      uiState.wave = gameState.current_level;
      uiState.phase = gameState.phase;
      uiState.result = gameState.game_result;

      gameEngine.draw_state(gameCanvas.getArt(), time);
      gameCanvas.drawState(gameState, uiState);
    },

    increaseAnimationSpeed() {
      uiState.animationSpeed = Math.min(uiState.animationSpeed + 1, 100);
    },
    decreaseAnimationSpeed() {
      uiState.animationSpeed = Math.max(uiState.animationSpeed - 1, 0);
    },
    handleStartButton() {
      gameEngine.start_wave();
    },
    handleSidebarClose() {
      uiState.selectedTower = null;
      uiState.selectedTurret = null;
    },
    handleTowerSell() {
      if (!uiState.selectedTower) {
        return;
      }
      gameEngine.sell_tower(uiState.selectedTower.turret_ref);
      uiState.selectedTower = null;
    },
    handleTowerUpgrade() {
      if (!uiState.selectedTower) {
        return;
      }

      // two click upgrading
      if (uiState.upgrading) {
        gameEngine.upgrade_tower(uiState.selectedTower.turret_ref);
        uiState.selectedTower = gameEngine.get_tower_by_ref(
          uiState.selectedTower.turret_ref
        );
        uiState.upgrading = false;
      } else {
        uiState.upgrading = true;
      }
    },
    getState() {
      return {
        uiState: JSON.parse(JSON.stringify(uiState)),
        gameEngine,
        gameCanvas: gameCanvas.getState(),
      };
    },
  };
}
