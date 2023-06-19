import { Animation, AnimationTemplate } from "./Animation";
import { EntityConfig } from "../game/entity/Entity";
import {
  movementToAnimationFromConfig,
  MovementToAnimationTemplateMap,
} from "./Movement";
import { SpriteSheet } from "./SpriteSheet";
import { Vector } from "./Vector";
import { AssetConfig } from "../game/Asset";
import { SPRITE_SCALE } from "../game/Constants";

export class Renderer {
  private readonly canvas: HTMLCanvasElement;
  private readonly ctx: CanvasRenderingContext2D;
  private spriteSheets: Record<string, SpriteSheet> = {};
  private animations: Record<string, AnimationTemplate> = {};
  private entityNameToMovmentAnimationMapMap: Record<
    string,
    MovementToAnimationTemplateMap
  > = {};

  constructor(canvas: HTMLCanvasElement) {
    this.canvas = canvas;
    this.ctx = canvas.getContext("2d")!;

    // @ts-ignore
    this.ctx.webkitImageSmoothingEnabled = false;
    // @ts-ignore
    this.ctx.mozImageSmoothingEnabled = false;
    this.ctx.imageSmoothingEnabled = false;
  }

  parseAssetConfig(assetConfig: AssetConfig) {
    const { name, asepriteFile } = assetConfig;

    const spriteSheet = SpriteSheet.fromFile(
      name,
      asepriteFile,
      assetConfig.spriteSheetFilePath
    );

    this.spriteSheets = {
      ...this.spriteSheets,
      [name]: spriteSheet,
    };

    this.animations = {
      ...this.animations,
      ...spriteSheet.getAnimationTemplates(),
    };
  }

  parseEntityConfig(entityConfig: EntityConfig) {
    const { name, movementAnimationsConfig } = entityConfig;

    this.parseAssetConfig(entityConfig);

    if (movementAnimationsConfig) {
      this.entityNameToMovmentAnimationMapMap = {
        ...this.entityNameToMovmentAnimationMapMap,
        [name]: movementToAnimationFromConfig(this, movementAnimationsConfig),
      };
    }
  }

  async loadAssets() {
    const promises = Object.values(this.spriteSheets).map((spriteSheet) =>
      spriteSheet.load()
    );

    await Promise.all(promises);

    console.log(this);
  }

  findSpriteSheet(name: string): SpriteSheet | null {
    return this.spriteSheets[name] ?? null;
  }

  findAnimationTemplate(id: string): AnimationTemplate | null {
    return this.animations[id] ?? null;
  }

  findMovementAnimationTemplateMap(
    entityName: string
  ): MovementToAnimationTemplateMap | null {
    return this.entityNameToMovmentAnimationMapMap[entityName] ?? null;
  }

  makeAnimation(
    id: string,
    currentTime: number,
    loop: boolean = false
  ): Animation | null {
    const template = this.findAnimationTemplate(id);

    if (!template) {
      return null;
    }
    return Animation.fromTemplate(template, currentTime, loop);
  }

  clear() {
    const { ctx } = this;
    const { width, height } = this.canvas;

    ctx.clearRect(0, 0, width, height);

    // ctx.fillStyle = "black";

    // ctx.fillRect(0, 0, width / 2, height / 2);
  }

  getSize() {
    return {
      width: this.canvas.width,
      height: this.canvas.height,
    };
  }

  saveTransform() {
    this.ctx.save();
  }

  restoreTransform() {
    this.ctx.restore();
  }

  translate(x: number, y: number) {
    this.ctx.translate(x, y);
  }

  rotate(angle: number) {
    this.ctx.rotate(angle);
  }

  scale(x: number, y: number) {
    this.ctx.scale(x, y);
  }

  renderCircle(
    position: Vector,
    radius: number,
    strokeColor: string,
    fillColor: string
  ) {
    const { ctx } = this;

    ctx.strokeStyle = strokeColor;
    ctx.fillStyle = fillColor;

    ctx.beginPath();
    ctx.arc(position.x, position.y, radius, 0, 2 * Math.PI);
    ctx.stroke();
    ctx.fill();
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

    const rotationInRadians = (rotation * Math.PI) / 180;

    ctx.save();
    ctx.translate(
      Math.floor(position.x * SPRITE_SCALE),
      Math.floor(position.y * SPRITE_SCALE)
    );
    ctx.rotate(rotationInRadians);
    ctx.drawImage(
      spriteSheet.image,
      Math.floor(sprite.frame.x),
      Math.floor(sprite.frame.y),
      Math.floor(sprite.frame.w),
      Math.floor(sprite.frame.h),
      Math.floor((-sprite.frame.w / 2.0) * SPRITE_SCALE),
      Math.floor((-sprite.frame.h / 2.0) * SPRITE_SCALE),
      Math.floor(sprite.frame.w * SPRITE_SCALE),
      Math.floor(sprite.frame.h * SPRITE_SCALE)
    );
    ctx.restore();
  }

  renderSpriteSheetFrame(
    spriteSheetId: string,
    frame: number,
    position: Vector,
    rotation: number
  ) {
    const spriteSheet = this.spriteSheets[spriteSheetId];

    if (!spriteSheet) {
      throw new Error(`No sprite sheet with id ${spriteSheetId}`);
    }

    const sprite = spriteSheet.findFrame(frame);

    if (!sprite) {
      throw new Error(`No sprite with frame ${frame}`);
    }

    this.renderSprite(spriteSheetId, sprite.id, position, rotation);
  }
}
