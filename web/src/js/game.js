const canvas = document.getElementById("canvas");

/** @type CanvasRenderingContext2D */
const ctx = canvas.getContext("2d");

function drawTurret(turret) {
  ctx.strokeStyle = "white";
  ctx.strokeRect(turret.x - 15, turret.y - 15, 30, 30);

  ctx.strokeStyle = "white";
  ctx.beginPath();
  ctx.moveTo(turret.x, turret.y);
  ctx.lineTo(
    turret.x + 15 * Math.cos(turret.rotation),
    turret.y + 15 * Math.sin(turret.rotation)
  );
  ctx.stroke();
}
function drawParticle(particle) {
  if (!particle.visible) {
    return;
  }
  ctx.fillStyle = "silver";
  ctx.strokeStyle = "silver";
  ctx.beginPath();
  ctx.arc(particle.x, particle.y, 5, 0, 2 * Math.PI);
  ctx.fill();
}
function drawCreep(creep) {
  ctx.fillStyle = "yellow";
  ctx.beginPath();
  ctx.moveTo(creep.x - 10, creep.y + 10);
  ctx.lineTo(creep.x, creep.y - 10);
  ctx.lineTo(creep.x + 10, creep.y + 10);
  ctx.lineTo(creep.x - 10, creep.y + 10);
  ctx.closePath();
  ctx.fill();

  const healthPercentage = creep.health / creep.maxHealth;

  ctx.fillStyle = "green";
  ctx.fillRect(creep.x - 10, creep.y - 12, 20 * healthPercentage, 2);
  ctx.fillStyle = "red";
  ctx.fillRect(
    creep.x - 10 + 20 * healthPercentage,
    creep.y - 12,
    20 * (1 - healthPercentage),
    2
  );
}

function drawState(state) {
  ctx.fillStyle = "black";
  ctx.fillRect(0, 0, canvas.clientWidth, canvas.clientHeight);

  for (const turret of state.turrets) {
    drawTurret(turret);
  }
  for (const creep of state.creeps) {
    drawCreep(creep);
  }
  for (const particle of state.particles) {
    drawParticle(particle);
  }
}

const state = {
  turrets: [
    { x: 50, y: 100, rotation: 0, lastShot: 0 },
    { x: 300, y: 100, rotation: 0, lastShot: 0 },
  ],
  creeps: [{ x: 0, y: 200, health: 10, maxHealth: 10 }],
  particles: [],
};

function updateState(time, state) {
  for (const creep of state.creeps) {
    creep.x += 1;
    if (creep.x > canvas.width) {
      creep.x = -10;
    }
  }

  for (const turret of state.turrets) {
    const targetCreep = state.creeps[0];
    const dx = targetCreep.x - turret.x;
    const dy = targetCreep.y - turret.y;
    const d = Math.sqrt(dx ** 2 + dy ** 2);
    turret.rotation = Math.acos(dx / d);
    if (time > turret.lastShot + 1000) {
      turret.lastShot = time;
      state.particles.push({
        x: turret.x + 15 * Math.cos(turret.rotation),
        y: turret.y + 15 * Math.sin(turret.rotation),
        visible: true,
        target: targetCreep,
      });
    }
  }

  for (const particle of state.particles) {
    if (!particle.visible) {
      continue;
    }
    const dx = particle.target.x - particle.x;
    const dy = particle.target.y - particle.y;
    const d = Math.sqrt(dx ** 2 + dy ** 2);
    if (d < 5) {
      particle.visible = false;
      particle.target.health = Math.max(particle.target.health - 1, 0);
    } else {
      particle.x += (dx / d) * 5;
      particle.y += (dy / d) * 5;
    }
  }
}

function loop(time) {
  updateState(time, state);
  drawState(state);
  requestAnimationFrame(loop);
}

requestAnimationFrame(loop);
