import { AnimationTemplate } from "../../../engine/Animation";
import { Vector } from "../../../engine/Vector";
import { Game } from "../../Game";
import { Entity, EntityAttributes } from "../Entity";

export class Plant extends Entity {
  protected readonly idleAnimationName: string;
  protected readonly activeAnimationName: string;

  constructor(
    name: string,
    position: Vector,
    rotation: number,
    velocity: Vector,
    angularVelocity: number,
    attributes: EntityAttributes,
    idleAnimationName: string,
    activeAnimationName: string
  ) {
    super(name, position, rotation, velocity, angularVelocity, attributes);
    this.idleAnimationName = idleAnimationName;
    this.activeAnimationName = activeAnimationName;
  }

  init(game: Game) {
    super.init(game);

    this.activeAnimation = game
      .getRenderer()
      .makeAnimation(this.idleAnimationName, game.getNow(), true);

    console.log({ activeAnimation: this.activeAnimation });
  }
}
