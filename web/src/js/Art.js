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
    this.canvas.strokeRect({
      x,
      y,
      width: turretSize,
      height: turretSize,
      color: "white",
    });

    const cannonLength = turretSize / 2;
    this.canvas.drawLine({
      start: {
        x: x + cannonLength,
        y: y + cannonLength,
      },
      end: {
        x: x + cannonLength * (1 + Math.cos(turret.rotation)),
        y: y + cannonLength * (1 + Math.sin(turret.rotation)),
      },
      color: "white",
    });
  }

  drawParticle(particle) {
    this.canvas.fillCircle({
      x: particle.pos.x,
      y: particle.pos.y,
      r: PARTICLE_SIZE,
      color: "silver",
    });
  }

  drawCreep(creep, current_level) {
    this.canvas.fillTriangle({
      x: creep.pos.x,
      y: creep.pos.y,
      size: CREEP_SIZE,
      color: current_level == "1" ? "yellow" : "red",
    });

    const healthPercentage = creep.health / creep.max_health;

    this.canvas.fillRect({
      x: creep.pos.x - CREEP_SIZE / 2,
      y: creep.pos.y - CREEP_SIZE / 2 - HEALTH_BAR_HEIGHT,
      width: CREEP_SIZE * healthPercentage,
      height: HEALTH_BAR_HEIGHT,
      color: "green",
    });
  }

  /**
   * @param {wasm.ExternalState} state
   */
  drawMap(state, time) {
    this.canvas.strokeRect({
      x: 0,
      y: 0,
      width: state.board_dimension_x,
      height: state.board_dimension_y,
      color: "white",
    });

    this.canvas.drawPath({
      points: state.creep_path,
      color: "white",
      segments: [3, 5],
      dashOffset: -time / 60,
    });

    this.canvas.fillRect({
      x: state.creep_spawn.x,
      y: state.creep_spawn.y,
      width: state.cell_length,
      height: state.cell_length,
      color: "rgba(0, 255, 0, 0.3)",
    });

    this.canvas.fillRect({
      x: state.creep_goal.x,
      y: state.creep_goal.y,
      width: state.cell_length,
      height: state.cell_length,
      color: "rgba(255, 0, 0, 0.3)",
    });
  }

  clear() {
    this.canvas.clear();
  }
}
