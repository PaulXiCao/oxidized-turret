import { Canvas } from "./Canvas.js";
import { Art } from "./Art.js";

const ui = new Canvas(document.getElementById("ui"));
ui.resize({ width: window.innerWidth, height: window.innerHeight });

const uiArt = new Art(ui);

window.addEventListener("resize", function () {
  ui.resize({ width: window.innerWidth, height: window.innerHeight });
  drawUi();
});

export function drawUi() {
  ui.fillRect({
    x: 0,
    y: 0,
    width: 50,
    height: ui.getSize().height,
    color: "#222222",
  });
  ui.fillRect({
    x: 5,
    y: 5,
    width: 40,
    height: 40,
    color: "black",
  });
  uiArt.drawTurret({ pos: { x: 10, y: 10 }, rotation: 0 }, 30);
}
