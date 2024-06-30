import { Canvas } from "./Canvas.js";

const PARTICLE_SIZE = 5;
const CREEP_SIZE = 20;
const HEALTH_BAR_HEIGHT = 2;

export class Art {
  /**
   * @param {Canvas} canvas
   */
  constructor(canvas) {
    this.canvas = canvas;
  }

  drawTurret(x, y, rotation, size, type) {
    let turretColor = ["yellow", "red", "cyan"][type];

    this.canvas.strokeRect(x, y, size, size, turretColor);

    const cannonLength = size / 2;
    this.canvas.drawLine(
      x + cannonLength,
      y + cannonLength,
      x + cannonLength * (1 + Math.cos(rotation)),
      y + cannonLength * (1 + Math.sin(rotation)),
      turretColor
    );
  }

  drawParticle(x, y) {
    this.canvas.fillCircle(x, y, PARTICLE_SIZE, "silver");
  }

  drawCannonParticle(x, y, r) {
    this.canvas.fillCircle(x, y, r, "orange");
  }

  drawCreep(x, y, healthPercentage, currentLevel) {
    this.canvas.fillTriangle(
      x,
      y,
      CREEP_SIZE,
      currentLevel == "1" ? "yellow" : "red"
    );

    this.canvas.fillRect(
      x - CREEP_SIZE / 2,
      y - CREEP_SIZE / 2 - HEALTH_BAR_HEIGHT,
      CREEP_SIZE * healthPercentage,
      HEALTH_BAR_HEIGHT,
      "green"
    );
  }

  drawMap(width, height) {
    this.canvas.strokeRect(0, 0, width, height, "white");
  }

  startCreepPath(x, y, time) {
    this.canvas.startPath(x, y, "white", -time / 60);
  }

  drawCreepPathLine(x, y) {
    this.canvas.drawPathLine(x, y);
  }

  endCreepPath() {
    this.canvas.endPath();
  }

  drawCreepSpawn(x, y, size) {
    this.canvas.fillRect(x, y, size, size, "rgba(0, 255, 0, 0.3)");
  }

  drawCreepGoal(x, y, size) {
    this.canvas.fillRect(x, y, size, size, "rgba(255, 0, 0, 0.3)");
  }

  clear() {
    this.canvas.clear();
  }
}
