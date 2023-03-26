import "./style.css";

const canvas = document.querySelector<HTMLCanvasElement>("#canvas");
const ctx = canvas?.getContext("2d");

function onWindowResize() {
  if (!canvas) {
    return;
  }

  canvas.width = window.innerWidth;
  canvas.height = window.innerHeight;
}

function tick() {
  if (!canvas || !ctx) {
    return;
  }

  ctx.clearRect(0, 0, canvas.width, canvas.height);

  ctx.fillStyle = "red";

  ctx.beginPath();
  ctx.arc(50, 50, 20, 0, Math.PI * 2, true);
  ctx.fill();

  window.requestAnimationFrame(tick);
}

window.addEventListener("resize", onWindowResize);
window.requestAnimationFrame(tick);

onWindowResize();
