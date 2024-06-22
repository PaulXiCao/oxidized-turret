import { ExternalTurret } from "../wasm/oxidized_turret_bg.js";
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

  /**
   *
   * @param {ExternalTurret} turret
   * @param {number} turretSize
   */
  drawTurret(turret, turretSize) {
    const x = turret.pos.x;
    const y = turret.pos.y;

    let turretColor = "white";
    if (turret.kind === 1) turretColor = "red";

    this.canvas.strokeRect(x, y, turretSize, turretSize, turretColor);

    const cannonLength = turretSize / 2;
    this.canvas.drawLine(
      x + cannonLength,
      y + cannonLength,
      x + cannonLength * (1 + Math.cos(turret.rotation)),
      y + cannonLength * (1 + Math.sin(turret.rotation)),
      "white"
    );
  }

  drawParticle(particle) {
    this.canvas.fillCircle(
      particle.pos.x,
      particle.pos.y,
      PARTICLE_SIZE,
      "silver"
    );
  }

  drawCreep(creep, current_level) {
    this.canvas.fillTriangle(
      creep.pos.x,
      creep.pos.y,
      CREEP_SIZE,
      current_level == "1" ? "yellow" : "red"
    );

    const healthPercentage = creep.health / creep.max_health;

    this.canvas.fillRect(
      creep.pos.x - CREEP_SIZE / 2,
      creep.pos.y - CREEP_SIZE / 2 - HEALTH_BAR_HEIGHT,
      CREEP_SIZE * healthPercentage,
      HEALTH_BAR_HEIGHT,
      "green"
    );
  }

  /**
   * @param {wasm.ExternalState} state
   */
  drawMap(state, time) {
    this.canvas.strokeRect(
      0,
      0,
      state.board_dimension_x,
      state.board_dimension_y,
      "white"
    );

    this.canvas.drawPath(state.creep_path, "white", -time / 60);

    this.canvas.fillRect(
      state.creep_spawn.x,
      state.creep_spawn.y,
      state.cell_length,
      state.cell_length,
      "rgba(0, 255, 0, 0.3)"
    );

    for (const goal of state.creep_goals) {
      this.canvas.fillRect(
        goal.x,
        goal.y,
        state.cell_length,
        state.cell_length,
        "rgba(255, 0, 0, 0.3)"
      );
    }
  }

  clear() {
    this.canvas.clear();
  }
}
