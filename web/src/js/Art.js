import { Canvas } from "./Canvas.js";
import { CreepKind } from "../wasm/oxidized_turret_bg.js";

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
    let turretColor = ["yellow", "red", "cyan", "green", "blue"][type];
    const ctx = this.canvas.canvas.getContext("2d");
    let lineWidth = ctx.lineWidth;
    ctx.lineWidth = 3;
    this.canvas.strokeRect(x + 2, y + 2, size - 4, size - 4, turretColor);
    if (type === 4) {
      this.canvas.fillCircle(
        x + 2 + (size - 4) / 2,
        y + 2 + (size - 4) / 2,
        size / 4,
        turretColor
      );
    } else {
      const cannonLength = size / 2;
      this.canvas.drawLine(
        x + cannonLength,
        y + cannonLength,
        x + cannonLength * (1 + Math.cos(rotation)),
        y + cannonLength * (1 + Math.sin(rotation)),
        turretColor
      );
    }
    ctx.lineWidth = lineWidth;
  }

  drawParticle(x, y) {
    this.canvas.fillCircle(x, y, PARTICLE_SIZE, "silver");
  }

  drawCannonParticle(x, y, r) {
    this.canvas.fillCircle(x, y, r, "orange");
  }

  drawSniperParticle(x, y, x2, y2) {
    this.canvas.drawLine(x, y, x2, y2, "white");
  }

  drawCreep(x, y, healthPercentage, kind) {
    if (kind === CreepKind.Normal) {
      this.canvas.fillCircle(x, y, CREEP_SIZE / 2.0, "green");
    } else if (kind === CreepKind.Grouped) {
      this.canvas.fillCircle(x, y, CREEP_SIZE / 4.0, "green");
    } else if (kind === CreepKind.Speed) {
      this.canvas.fillTriangle(x, y, CREEP_SIZE, "yellow");
    } else if (kind === CreepKind.Big) {
      this.canvas.fillRect(
        x - CREEP_SIZE / 2.0,
        y - CREEP_SIZE / 2.0,
        CREEP_SIZE,
        CREEP_SIZE,
        "red"
      );
    } else {
      throw new Error(`Uninmplemented kreep kind '${kind}'...`);
    }

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
