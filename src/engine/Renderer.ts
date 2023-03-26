import {
  AnimationDefinition,
  AnimationTemplate,
  SpriteSheetAndAnimations,
} from "./Animation";
import { Game } from "./Game";
import { SpriteSheet } from "./SpriteSheet";
import { Vector } from "./Vector";
import { World } from "./World";

export class Renderer {
  readonly canvas: HTMLCanvasElement;
  readonly ctx: CanvasRenderingContext2D;
  readonly spriteSheets: Record<string, SpriteSheet> = {};
  animations: Record<string, AnimationTemplate> = {};

  constructor(canvas: HTMLCanvasElement) {
    this.canvas = canvas;
    this.ctx = canvas.getContext("2d")!;
  }

  addSpriteSheetAndAnimations(
    id: string,
    SpriteSheetAndAnimations: SpriteSheetAndAnimations
  ) {
    const { spriteSheet, animations } = SpriteSheetAndAnimations;

    this.spriteSheets[id] = spriteSheet;
    this.animations = { ...this.animations, ...animations };
  }

  async loadSpriteSheets() {
    const promises = Object.values(this.spriteSheets).map((spriteSheet) =>
      spriteSheet.load()
    );

    await Promise.all(promises);
  }

  findAnimation(id: string): AnimationTemplate | null {
    return this.animations[id] ?? null;
  }

  renderWorld(game: Game, world: World, now: number) {
    const { ctx } = this;
    const { width, height } = this.canvas;
    const { camera } = game;

    ctx.clearRect(0, 0, width, height);

    ctx.fillStyle = "black";

    ctx.fillRect(0, 0, width / 2, height / 3);

    ctx.save();

    ctx.translate(width / 2, height / 2);

    ctx.scale(camera.scale, camera.scale);

    for (const entity of world.entities) {
      //   entity.render(this);
    }

    world.player.render(this, now);

    ctx.restore();
  }

  renderSprite(
    spriteSheetId: string,
    spriteId: string,
    position: Vector,
    rotation: number
  ) {
    const { ctx } = this;

    const spriteSheet = this.spriteSheets[spriteSheetId];

    if (!spriteSheet) {
      throw new Error(`No sprite sheet with id ${spriteSheetId}`);
    }

    const sprite = spriteSheet.sprites[spriteId];

    if (!sprite) {
      throw new Error(`No sprite with id ${spriteId}`);
    }

    if (!spriteSheet.image) {
      return;
    }

    const { rotationPoint, spriteSize } = spriteSheet;
    const { sx, sy } = sprite;

    const rotationInRadians = (rotation * Math.PI) / 180;

    ctx.save();
    ctx.translate(
      position.x - spriteSize.width / 2,
      position.y - spriteSize.height / 2
    );
    ctx.rotate(rotationInRadians);
    ctx.fillStyle = "red";
    ctx.fillRect(
      -rotationPoint.x,
      -rotationPoint.y,
      spriteSize.width,
      spriteSize.height
    );
    ctx.drawImage(
      spriteSheet.image,
      sx * spriteSize.width,
      sy * spriteSize.height,
      spriteSize.width,
      spriteSize.height,
      -rotationPoint.x,
      -rotationPoint.y,
      spriteSize.width,
      spriteSize.height
    );
    ctx.restore();
  }
}
