import { Canvas } from "./Canvas.js";
import { Art } from "./Art.js";
import {
  TurretRef,
  GamePhase,
  GameResult,
} from "../wasm/oxidized_turret_bg.js";

function drawBasicTurret(uiCanvas, uiArt, uiState) {
  // draw selection
  if (uiState.selectedTurret === 0) {
    uiCanvas.fillRect(0, 0, 50, 50, "green");
  }

  // draw icon
  uiArt.drawTurret(10, 10, 0, 30, 0);
}

function drawSniperTurret(uiCanvas, uiArt, uiState) {
  // draw selection
  if (uiState.selectedTurret === 1) {
    uiCanvas.fillRect(0, 50, 50, 50, "green");
  }

  // draw icon
  uiArt.drawTurret(10, 60, 10, 30, 1);
}

function drawCannonTurret(uiCanvas, uiArt, uiState) {
  // draw selection
  if (uiState.selectedTurret === 2) {
    uiCanvas.fillRect(0, 100, 50, 50, "green");
  }

  // draw icon
  uiArt.drawTurret(10, 110, 90, 30, 2);
}

function drawMultiTurret(uiCanvas, uiArt, uiState) {
  // draw selection
  if (uiState.selectedTurret === 3) {
    uiCanvas.fillRect(0, 150, 50, 50, "green");
  }

  // draw icon
  uiArt.drawTurret(10, 160, 90, 30, 3);
}

function round(num) {
  return Math.round(num * 10) / 10;
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
  towerUpgradeButton,
}) {
  const uiCanvas = new Canvas(canvas);
  const uiArt = new Art(uiCanvas);

  return {
    drawUi(uiState) {
      // draw background
      uiCanvas.fillRect(0, 0, 50, uiCanvas.getSize().height, "#222222");
      drawBasicTurret(uiCanvas, uiArt, uiState);
      drawSniperTurret(uiCanvas, uiArt, uiState);
      drawCannonTurret(uiCanvas, uiArt, uiState);
      drawMultiTurret(uiCanvas, uiArt, uiState);

      if (uiState.selectedTower) {
        /** @type {TurretRef} */
        const turret = uiState.selectedTower;
        towerDetailSidebar.style.display = "block";

        if (turret.data.stats.length === turret.data.next_stats.length) {
          towerUpgradeButton.style.display = "inline-block";
        } else {
          towerUpgradeButton.style.display = "none";
        }

        if (uiState.upgrading) {
          towerUpgradeButton.innerText = "Buy Upgrade!";
        } else {
          towerUpgradeButton.innerText = "Upgrade?";
        }

        if (
          uiState.upgrading &&
          turret.data.stats.length === turret.data.next_stats.length
        ) {
          var towerStatsTable = `
          <table>
          <tr><th>Metric</th><th>Now</th><th>Next</th></tr>
          ${turret.data.stats
            .map((stat, index) => {
              const next = turret.data.next_stats[index];
              return `<tr><td>${stat.key} ${stat.unit}</td><td>${round(
                stat.value
              )}</td><td>${round(next.value)}</td></tr>`;
            })
            .join("")}
          </table>`;
        } else {
          var towerStatsTable = `
          <table>
          <tr><th>Metric</th><th>Value</th></tr>
          ${turret.data.stats
            .map(
              (stat) =>
                `<tr><td>${stat.key}</td><td>${round(stat.value)} ${
                  stat.unit
                }</td></tr>`
            )
            .join("")}
          </table>`;
        }

        towerStats.innerHTML = `
          <div>Selected Tower (id): ${turret.turret_ref.id}</div>
          ${towerStatsTable}
        `;
      } else {
        towerDetailSidebar.style.display = "none";
      }

      uiCanvas.fillCircle(10, 40, 7, "gray");
      uiCanvas.fillText(7, 44, "1", "black", 12);

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
      uiCanvas.resize(width, height);
    },
  };
}
