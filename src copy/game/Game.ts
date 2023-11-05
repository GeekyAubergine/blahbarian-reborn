import { Camera } from "./Camera";
import { EntityAttributes, EntityConfig } from "./entity/Entity";
import { Event } from "./Events";
import { Keyboard } from "../engine/Keyboard";
import { Renderer } from "../engine/Renderer";
import { Vector } from "../engine/Vector";
import { Scene } from "./scene/Scene";
import { ENTITY_NAMES } from "./Constants";
import { Animation } from "../engine/Animation";
import { BattlefieldScene } from "./scene/BattlefieldScene";
import { ASSETS } from "./Asset";

export class Game {
  private renderer: Renderer;
  private currentScene: Scene;
  private keyboard: Keyboard;

  private lastTick: number = 0;
  private now: number = 0;
  private camera: Camera = new Camera(new Vector(0, 0), 0.5);
  private events: Event[] = [];
  private entityDefaultAttributes: Record<ENTITY_NAMES, EntityAttributes> = {};

  private testAnim: Animation | null = null;
  private testAnim2: Animation | null = null;

  constructor(canvas: HTMLCanvasElement, entityConfits: EntityConfig[]) {
    this.renderer = new Renderer(canvas);
    this.keyboard = new Keyboard();

    for (const config of ASSETS) {
      this.renderer.parseAssetConfig(config);
    }

    for (const config of entityConfits) {
      this.parseEntityConfig(config);
    }

    this.currentScene = new BattlefieldScene();
  }

  parseEntityConfig(config: EntityConfig) {
    this.renderer.parseEntityConfig(config);
    this.entityDefaultAttributes[config.name] = config.attributes;
  }

  async loadAssets() {
    await this.renderer.loadAssets();
  }

  init() {
    this.currentScene.init(this);
  }

  dispatchEvent(event: Event) {
    this.events.push(event);
  }

  update() {
    this.keyboard.update();
    if (this.lastTick === 0) {
      this.lastTick = Date.now() / 1000;
    }
    this.now = Date.now() / 1000;
    const dt = this.now - this.lastTick;
    const eventsToProcess = this.events.slice();

    if (this.testAnim == null) {
      this.testAnim = this.renderer.makeAnimation(
        "seagrass-idle",
        this.now,
        true
      );
    }

    if (this.testAnim2 == null) {
      this.testAnim2 = this.renderer.makeAnimation(
        "seagrass-the-second",
        this.now,
        true
      );
    }

    this.events = [];
    // for (const event of eventsToProcess) {
    //   this.world.onEvent(this, event);
    // }
    // this.world.update(this, dt);
    // this.camera.update(this, dt);
    this.lastTick = this.now;
  }

  render() {
    this.renderer.clear();
    this.currentScene.render(this, this.now);
    // this.renderer.renderSprite("seagrass", "seagrass-0", new Vector(0, 0), 0);

    // if (this.testAnim) {
    //   this.testAnim.render(this.renderer, this.now, new Vector(64, 0), 0);
    // }

    // if (this.testAnim2) {
    //   this.testAnim2.render(this.renderer, this.now, new Vector(64, 64), 0);
    // }
  }

  getNow() {
    return this.now;
  }

  getRenderer() {
    return this.renderer;
  }

  getWorld() {
    return this.currentScene;
  }

  getCamera() {
    return this.camera;
  }

  getEntityDefaultAttributes(name: ENTITY_NAMES): EntityAttributes {
    return this.entityDefaultAttributes[name];
  }

  getKeyboard() {
    return this.keyboard;
  }
}
