import { Canvas } from "./Canvas.js";
import { Art } from "./Art.js";

const uiCanvas = new Canvas(document.getElementById("ui"));
uiCanvas.resize({ width: window.innerWidth, height: window.innerHeight });

const uiArt = new Art(uiCanvas);

const uiState = new Proxy(
  {
    state: "initial",
    selectedTurret: null,
  },
  {
    set(target, propertyKey, value, receiver) {
      const result = Reflect.set(target, propertyKey, value, receiver);
      ui.drawUi();
      return result;
    },
  }
);

window.addEventListener("keyup", function shortcutHandler(event) {
  if (event.key === "1") {
    uiState.selectedTurret = uiState.selectedTurret === null ? 0 : null;
  }
});

export const ui = {
  drawUi() {
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
  },

  /**
   * @param {MouseEvent} event
   */
  handleMousedown(event) {
    return false;
  },
  /**
   * @param {MouseEvent} event
   */
  handleMouseup(event) {
    return false;
  },
  /**
   * @param {MouseEvent} event
   */
  handleClick(event) {
    if (event.clientX <= 50) {
      if (uiState.selectedTurret === null) {
        uiState.selectedTurret = 0;
      } else {
        uiState.selectedTurret = null;
      }
    }

    if (event.clientX > 50 && uiState.selectedTurret !== null) {
      window.dispatchEvent(
        new CustomEvent("buildTower", {
          detail: {
            type: uiState.selectedTurret,
            screenPos: { x: event.clientX, y: event.clientY },
          },
        })
      );
    }

    return false;
  },
  getState() {
    return uiState;
  },
};

window.addEventListener("resize", function () {
  uiCanvas.resize({ width: window.innerWidth, height: window.innerHeight });
  ui.drawUi();
});
