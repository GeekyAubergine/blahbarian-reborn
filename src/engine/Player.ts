import { Animation } from "./Animation";
import { Entity } from "./Entity";
import { Event } from "./Events";
import { Game } from "./Game";
import { Renderer } from "./Renderer";
import { World } from "./World";

export class Player extends Entity {
  animation: Animation | null = null;
  idleAnimation: Animation | null = null;
  downAnimation: Animation | null = null;
  upAnimation: Animation | null = null;
  leftAnimation: Animation | null = null;
  rightAnimation: Animation | null = null;

  init(game: Game, world: World, renderer: Renderer, now: number) {
    const idleAnimationTemplate = renderer.findAnimation("shark-idle");
    const downAnimationTemplate = renderer.findAnimation("shark-down");
    const upAnimationTemplate = renderer.findAnimation("shark-up");
    const leftAnimationTemplate = renderer.findAnimation("shark-left");
    const rightAnimationTemplate = renderer.findAnimation("shark-right");

    if (!idleAnimationTemplate) {
      throw new Error("Could not find shark-idle animation");
    }

    if (!downAnimationTemplate) {
      throw new Error("Could not find shark-down animation");
    }

    if (!upAnimationTemplate) {
      throw new Error("Could not find shark-up animation");
    }

    if (!leftAnimationTemplate) {
      throw new Error("Could not find shark-left animation");
    }

    if (!rightAnimationTemplate) {
      throw new Error("Could not find shark-right animation");
    }

    this.animation = Animation.fromTemplate(idleAnimationTemplate, now);
    this.idleAnimation = this.animation;
    this.downAnimation = Animation.fromTemplate(downAnimationTemplate, now);
    this.upAnimation = Animation.fromTemplate(upAnimationTemplate, now);
    this.leftAnimation = Animation.fromTemplate(leftAnimationTemplate, now);
    this.rightAnimation = Animation.fromTemplate(rightAnimationTemplate, now);
  }

  update(game: Game, world: World, dt: number, events: Event[]) {
    super.update(game, world, dt, events);

    if (this.animation == null) {
      this.animation = this.idleAnimation;
    }
  }

  // ...
  render(renderer: Renderer, now: number): void {
    if (this.animation) {
      this.animation.render(renderer, now, this.position, this.rotation);
    }
  }
}
