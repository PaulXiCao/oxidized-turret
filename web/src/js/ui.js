import { Canvas } from "./Canvas.js";
import { Art } from "./Art.js";
import {
  TurretRef,
  GamePhase,
  GameResult,
} from "../wasm/oxidized_turret_bg.js";

function drawBasicTurret(uiCanvas, uiArt, uiState) {
  // draw background
  uiCanvas.fillRect({
    x: 0,
    y: 0,
    width: 50,
    height: uiCanvas.getSize().height,
    color: "#222222",
  });

  // draw selection
  if (uiState.selectedTurret === 0) {
    uiCanvas.fillRect({
      x: 0,
      y: 0,
      width: 50,
      height: 50,
      color: "green",
    });
  }

  // draw icon
  uiArt.drawTurret({ pos: { x: 10, y: 10 }, rotation: 0, kind: 0 }, 30);
}

function drawSniperTurret(uiCanvas, uiArt, uiState) {
  // draw background
  uiCanvas.fillRect({
    x: 0,
    y: 50,
    width: 50,
    height: uiCanvas.getSize().height,
    color: "#222222",
  });

  // draw selection
  if (uiState.selectedTurret === 1) {
    uiCanvas.fillRect({
      x: 0,
      y: 50,
      width: 50,
      height: 50,
      color: "green",
    });
  }

  // draw icon
  uiArt.drawTurret({ pos: { x: 10, y: 60 }, rotation: 10, kind: 1 }, 30);
}

export function createUi({
  canvas,
  health,
  wave,
  gold,
  speed,
  global,
  result,
  towerDetailSidebar,
  towerStats,
}) {
  const uiCanvas = new Canvas(canvas);
  const uiArt = new Art(uiCanvas);

  return {
    drawUi(uiState) {
      drawBasicTurret(uiCanvas, uiArt, uiState);
      drawSniperTurret(uiCanvas, uiArt, uiState);

      if (uiState.selectedTower) {
        /** @type {TurretRef} */
        const turret = uiState.selectedTower;
        towerDetailSidebar.style.display = "block";
        towerStats.innerHTML = `
          <div>Selected Tower: ${turret.turret_ref.id}</div>
          <div>Range: ${turret.turret.range}</div>
        `;
      } else {
        towerDetailSidebar.style.display = "none";
      }

      uiCanvas.fillCircle({ x: 10, y: 40, r: 7, color: "gray" });
      uiCanvas.fillText({
        x: 7,
        y: 44,
        fontSize: 12,
        text: "1",
        color: "black",
      });

      health.innerText = `Health: ${uiState.health}`;
      wave.innerText = `Wave: ${uiState.wave}`;
      gold.innerText = `Gold: ${uiState.gold}`;
      speed.innerText = `Speed: ${uiState.animationSpeed}`;

      global.className = "";
      if (uiState.phase === GamePhase.Building) {
        global.classList.add("building");
      } else {
        global.classList.add("fighting");
      }

      switch (uiState.result) {
        case GameResult.PlayerWon: {
          result.style.display = "block";
          result.classList.add("won");
          result.innerText = `You won with ${uiState.health} hp!`;
          break;
        }
        case GameResult.CreepsWon: {
          result.style.display = "block";
          result.classList.add("lost");
          result.innerText = `You lost at level ${uiState.wave}!`;
          break;
        }
        case GameResult.StillRunning: {
          result.style.display = "none";
          break;
        }
      }
    },
    handleResize({ width, height }) {
      uiCanvas.resize({ width, height });
    },
  };
}
