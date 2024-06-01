import { Canvas } from "./Canvas.js";
import { Art } from "./Art.js";

const uiCanvas = new Canvas(document.getElementById("ui"));
uiCanvas.resize({ width: window.innerWidth, height: window.innerHeight });

const uiArt = new Art(uiCanvas);

const uiState = new Proxy(
  {
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

let mousedownObject = null;

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
  },

  /**
   * @param {MouseEvent} event
   */
  handleMousedown(event) {
    // sidebar
    if (event.clientX <= 50) {
      if (event.clientY >= 0 && event.clientY <= 50) {
        mousedownObject = 0;
      }
      return true;
    }
    return false;
  },
  /**
   * @param {MouseEvent} event
   */
  handleMouseup(event) {
    if (event.clientX <= 50 && mousedownObject !== null) {
      if (uiState.selectedTurret === null) {
        uiState.selectedTurret = mousedownObject;
      } else {
        uiState.selectedTurret = null;
      }

      mousedownObject = null;
      return true;
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
