import { Renderer } from "./Renderer";
import { Vector } from "./Vector";

export type AnimationTemplateFrame = {
  readonly id: string;
  readonly duration: number;
};

export interface AnimationTemplate {
  readonly id: string;
  readonly spriteSheetId: string;
  readonly frames: AnimationTemplateFrame[];
  readonly totalDuration: number;
}

export class Animation {
  readonly template: AnimationTemplate;
  startTime: number;
  readonly loop: boolean = true;

  constructor(template: AnimationTemplate, startTime: number, loop: boolean) {
    this.template = template;
    this.startTime = startTime;
    this.loop = loop;
  }

  static fromTemplate(
    template: AnimationTemplate,
    startTime: number,
    loop: boolean
  ): Animation {
    return new Animation(template, startTime, loop);
  }

  isDone(currentTime: number): boolean {
    const runTime = currentTime - this.startTime;
    return runTime > this.template.totalDuration;
  }

  private getFrame(currentTime: number): AnimationTemplateFrame | null {
    let runTime = (currentTime - this.startTime);

    let previousFrameStart = 0;
    for (let i = 0; i < this.template.frames.length; i++) {
      const frame = this.template.frames[i];
      const frameDuration = frame.duration / 1000;
      const frameEnd = previousFrameStart + frameDuration;

      if (runTime >= previousFrameStart && runTime < frameEnd) {
        return frame;
      }

      previousFrameStart = frameEnd;
    }

    if (this.loop) {
      this.startTime = currentTime;
      return this.getFrame(currentTime);
    }

    return null;
  }

  render(
    renderer: Renderer,
    currentTime: number,
    position: Vector,
    rotation: number
  ) {
    const frame = this.getFrame(currentTime);
    if (frame) {
      renderer.renderSprite(
        this.template.spriteSheetId,
        frame.id,
        position,
        rotation
      );
    }
  }
}
