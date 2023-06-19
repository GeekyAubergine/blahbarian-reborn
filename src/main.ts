import "./style.css";
import { Game } from "./game/Game";
import { SEA_GRASS_CONFIG } from "./assets/plants/seagrass";

const ENTITIES = [SEA_GRASS_CONFIG];

const canvas = document.querySelector<HTMLCanvasElement>("#canvas");
let game: Game | null = null;

function onWindowResize() {
  if (!canvas) {
    return;
  }

  canvas.width = window.innerWidth;
  canvas.height = window.innerHeight;
}

async function initialize(): Promise<void> {
  if (!canvas) {
    throw new Error("No canvas");
  }

  const g = new Game(canvas, ENTITIES);

  await g.loadAssets();

  await g.init();

  game = g;
}

function tick() {
  if (!game) {
    window.requestAnimationFrame(tick);
    return;
  }

  game.update();

  game.render();

  window.requestAnimationFrame(tick);
}

window.addEventListener("resize", onWindowResize);
window.requestAnimationFrame(tick);

onWindowResize();

initialize();
