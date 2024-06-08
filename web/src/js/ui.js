import { Canvas } from "./Canvas.js";
import { Art } from "./Art.js";

export function createUi({ canvas, health, wave, gold }) {
  const uiCanvas = new Canvas(canvas);
  const uiArt = new Art(uiCanvas);
  return {
    drawUi(uiState) {
      uiCanvas.fillRect({
        x: 0,
        y: 0,
        width: 50,
        height: uiCanvas.getSize().height,
        color: "#222222",
      });
      if (uiState.selectedTurret === 0) {
        uiCanvas.fillRect({
          x: 0,
          y: 0,
          width: 50,
          height: 50,
          color: "green",
        });
      }
      uiCanvas.fillRect({
        x: 5,
        y: 5,
        width: 40,
        height: 40,
        color: "black",
      });
      uiArt.drawTurret({ pos: { x: 10, y: 10 }, rotation: 0 }, 30);
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
    },
    handleResize({ width, height }) {
      uiCanvas.resize({ width, height });
    },
  };
}
