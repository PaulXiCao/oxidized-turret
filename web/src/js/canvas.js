const canvas = document.getElementById("canvas");
canvas.width = window.innerWidth;
canvas.height = window.innerHeight;

window.addEventListener("resize", function () {
  canvas.width = window.innerWidth;
  canvas.height = window.innerHeight;
});

let scale = 1.0;
let offsetX = 0;
let offsetY = 0;

window.addEventListener("wheel", (event) => {
  scale = Math.min(Math.max(scale + 0.02 * Math.sign(event.deltaY), 0.25), 2);
});

let startMoveX = 0;
let startMoveY = 0;
let startOffsetX = 0;
let startOffsetY = 0;

function mousemove(event) {
  offsetX = startOffsetX + event.clientX - startMoveX;
  offsetY = startOffsetY + event.clientY - startMoveY;
}

window.addEventListener("mousedown", (event) => {
  startMoveX = event.clientX;
  startMoveY = event.clientY;
  startOffsetX = offsetX;
  startOffsetY = offsetY;
  window.addEventListener("mousemove", mousemove);
});

window.addEventListener("mouseup", (event) => {
  window.removeEventListener("mousemove", mousemove);
});

/** @type CanvasRenderingContext2D */
const ctx = canvas.getContext("2d");

export function strokeRect({ x, y, width, height, color }) {
  ctx.strokeStyle = color;
  ctx.strokeRect(
    x / scale + offsetX,
    y / scale + offsetY,
    width / scale,
    height / scale
  );
}

export function fillRect({ x, y, width, height, color }) {
  ctx.fillStyle = color;
  ctx.fillRect(
    x / scale + offsetX,
    y / scale + offsetY,
    width / scale,
    height / scale
  );
}

export function drawLine({ start, end, color }) {
  ctx.strokeStyle = color;
  ctx.beginPath();
  ctx.moveTo(start.x / scale + offsetX, start.y / scale + offsetY);
  ctx.lineTo(end.x / scale + offsetX, end.y / scale + offsetY);
  ctx.stroke();
}

export function drawPath({ points, color, segments = [], dashOffset = 0 }) {
  ctx.strokeStyle = color;
  ctx.beginPath();
  ctx.setLineDash(segments);
  ctx.lineDashOffset = dashOffset;

  const start = points[0];
  ctx.moveTo(start.x / scale + offsetX, start.y / scale + offsetY);
  for (let i = 1; i < points.length; i += 1) {
    ctx.lineTo(points[i].x / scale + offsetX, points[i].y / scale + offsetY);
  }
  ctx.stroke();
  // reset line dash
  ctx.setLineDash([]);
}

export function fillCircle({ x, y, r, color }) {
  ctx.fillStyle = color;
  ctx.beginPath();
  ctx.arc(x / scale + offsetX, y / scale + offsetY, r / scale, 0, 2 * Math.PI);
  ctx.fill();
}

export function fillTriangle({ x, y, size, color }) {
  ctx.fillStyle = color;
  ctx.beginPath();
  ctx.moveTo(
    (x - size / 2) / scale + offsetX,
    (y + size / 2) / scale + offsetY
  );
  ctx.lineTo(x / scale + offsetX, (y - size / 2) / scale + offsetY);
  ctx.lineTo(
    (x + size / 2) / scale + offsetX,
    (y + size / 2) / scale + offsetY
  );
  ctx.lineTo(
    (x - size / 2) / scale + offsetX,
    (y + size / 2) / scale + offsetY
  );
  ctx.closePath();
  ctx.fill();
}

export function clear() {
  ctx.clearRect(0, 0, canvas.width, canvas.height);
}
