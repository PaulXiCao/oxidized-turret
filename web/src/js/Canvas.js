export class Canvas {
  /**
   * @param {HTMLCanvasElement} canvas
   * @param {object} options
   * @param {number} options.scale
   * @param {number} options.offsetX
   * @param {number} options.offsetY
   */
  constructor(canvas, { scale = 1.0, offsetX = 0.0, offsetY = 0.0 } = {}) {
    this.canvas = canvas;
    /** @type {CanvasRenderingContext2D} */
    this.ctx = canvas.getContext("2d");
    this.scale = scale;
    this.offsetX = offsetX;
    this.offsetY = offsetY;
  }

  getOffset() {
    return { x: this.offsetX, y: this.offsetY };
  }

  /**
   * @param {number} x
   * @param {number} y
   */
  setOffset(x, y) {
    this.offsetX = x;
    this.offsetY = y;
  }

  getScale() {
    return this.scale;
  }

  realToCanvas({ x, y }) {
    return {
      x: (x - this.offsetX) * this.scale,
      y: (y - this.offsetY) * this.scale,
    };
  }

  /**
   * @param {number} scale
   */
  setScale(scale) {
    this.scale = scale;
  }

  strokeRect(x, y, width, height, color) {
    this.ctx.strokeStyle = color;
    this.ctx.strokeRect(
      x / this.scale + this.offsetX,
      y / this.scale + this.offsetY,
      width / this.scale,
      height / this.scale
    );
  }

  fillRect(x, y, width, height, color) {
    this.ctx.fillStyle = color;
    this.ctx.fillRect(
      x / this.scale + this.offsetX,
      y / this.scale + this.offsetY,
      width / this.scale,
      height / this.scale
    );
  }

  drawLine(startX, startY, endX, endY, color) {
    this.ctx.strokeStyle = color;
    this.ctx.beginPath();
    this.ctx.moveTo(
      startX / this.scale + this.offsetX,
      startY / this.scale + this.offsetY
    );
    this.ctx.lineTo(
      endX / this.scale + this.offsetX,
      endY / this.scale + this.offsetY
    );
    this.ctx.stroke();
  }

  startPath(x, y, color, dashOffset) {
    this.ctx.strokeStyle = color;
    this.ctx.beginPath();
    this.ctx.setLineDash([3, 5]);
    this.ctx.lineDashOffset = dashOffset;

    this.ctx.moveTo(
      x / this.scale + this.offsetX,
      y / this.scale + this.offsetY
    );
  }
  drawPathLine(x, y) {
    this.ctx.lineTo(
      x / this.scale + this.offsetX,
      y / this.scale + this.offsetY
    );
  }
  endPath() {
    this.ctx.stroke();
    // reset line dash
    this.ctx.setLineDash([]);
  }

  drawPath(points, color, dashOffset) {
    this.ctx.strokeStyle = color;
    this.ctx.beginPath();
    this.ctx.setLineDash([3, 5]);
    this.ctx.lineDashOffset = dashOffset;

    const start = points[0];
    this.ctx.moveTo(
      start.x / this.scale + this.offsetX,
      start.y / this.scale + this.offsetY
    );
    for (let i = 1; i < points.length; i += 1) {
      this.ctx.lineTo(
        points[i].x / this.scale + this.offsetX,
        points[i].y / this.scale + this.offsetY
      );
    }
    this.ctx.stroke();
    // reset line dash
    this.ctx.setLineDash([]);
  }

  fillCircle(x, y, r, color) {
    this.ctx.fillStyle = color;
    this.ctx.beginPath();
    this.ctx.arc(
      x / this.scale + this.offsetX,
      y / this.scale + this.offsetY,
      r / this.scale,
      0,
      2 * Math.PI
    );
    this.ctx.fill();
  }

  fillText(x, y, text, color, fontSize) {
    this.ctx.fillStyle = color;
    this.ctx.font = `bold ${fontSize}px Courier`;
    this.ctx.fillText(text, x, y);
  }

  fillTriangle(x, y, size, color) {
    this.ctx.fillStyle = color;
    this.ctx.beginPath();
    this.ctx.moveTo(
      (x - size / 2) / this.scale + this.offsetX,
      (y + size / 2) / this.scale + this.offsetY
    );
    this.ctx.lineTo(
      x / this.scale + this.offsetX,
      (y - size / 2) / this.scale + this.offsetY
    );
    this.ctx.lineTo(
      (x + size / 2) / this.scale + this.offsetX,
      (y + size / 2) / this.scale + this.offsetY
    );
    this.ctx.lineTo(
      (x - size / 2) / this.scale + this.offsetX,
      (y + size / 2) / this.scale + this.offsetY
    );
    this.ctx.closePath();
    this.ctx.fill();
  }

  clear() {
    this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);
  }

  getSize() {
    return { width: this.canvas.width, height: this.canvas.height };
  }

  resize(width, height) {
    this.canvas.width = width;
    this.canvas.height = height;
  }

  getState() {
    return { scale: this.scale, offsetX: this.offsetX, offsetY: this.offsetY };
  }
}
