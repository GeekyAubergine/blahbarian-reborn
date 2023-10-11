import { AnimationTemplate, AnimationTemplateFrame } from "./Animation";

type AsepriteFrame = {
  frame: {
    x: number;
    y: number;
    w: number;
    h: number;
  };
  spriteSourceSize: {
    x: number;
    y: number;
    w: number;
    h: number;
  };
  sourceSize: {
    w: number;
    h: number;
  };
  duration: number;
};

export type Sprite = {
  id: string;
} & AsepriteFrame;

type FrameTag = {
  name: string;
  from: number;
  to: number;
  direction: string;
};

type AsepriteMeta = {
  image: string;
  format: string;
  size: {
    w: number;
    h: number;
  };
  scale: string;
  frameTags: FrameTag[];
};

export type AsepriteFile = {
  frames: Record<string, AsepriteFrame>;
  meta: AsepriteMeta;
};

function cleanSpriteName(name: string): string {
  return name.replace(".aseprite", "");
}

export class SpriteSheet {
  id: string;
  readonly sprites: Record<string, Sprite>;
  readonly meta: AsepriteMeta;

  readonly assetFileName: string;
  image: ImageBitmap | null = null;

  constructor(
    id: string,
    sprites: Record<string, Sprite>,
    meta: AsepriteMeta,
    assetFileName: string
  ) {
    this.id = id;
    this.sprites = sprites;
    this.meta = meta;
    this.assetFileName = assetFileName;
  }

  static fromFile(
    id: string,
    definition: AsepriteFile,
    assetFileName: string
  ): SpriteSheet {
    const sprites = Object.entries(definition.frames).reduce<
      Record<string, Sprite>
    >((acc, [key, frame]) => {
      const fixedKey = cleanSpriteName(key);
      return {
        ...acc,
        [fixedKey]: {
          id: fixedKey,
          ...frame,
        },
      };
    }, {});

    return new SpriteSheet(id, sprites, definition.meta, assetFileName);
  }

  async load() {
    const image = new Image();
    image.src = this.assetFileName;
    return new Promise<void>((resolve, reject) => {
      image.onload = () => {
        createImageBitmap(image)
          .then((imageBitmap) => {
            console.log({ imageBitmap });
            this.image = imageBitmap;
            resolve();
          })
          .catch((error) => {
            console.error(error);
            reject(error);
          });
      };
    });
  }

  findSprite(id: string): Sprite | null {
    return this.sprites[id] ?? null;
  }

  findFrame(frame: number): Sprite | null {
    return this.findSprite(`${this.id}-${frame}`);
  }

  getAnimationTemplates(): Record<string, AnimationTemplate> {
    return this.meta.frameTags.reduce<Record<string, AnimationTemplate>>(
      (acc, tag) => {
        const frames = Object.entries(this.sprites)
          .slice(tag.from, tag.to + 1)
          .map(
            ([key, frame]): AnimationTemplateFrame => ({
              id: cleanSpriteName(key),
              duration: frame.duration,
            })
          );

        const totalDuration = frames.reduce(
          (acc, key) => acc + this.sprites[key.id].duration,
          0
        );

        return {
          ...acc,
          [tag.name]: {
            id: tag.name,
            spriteSheetId: this.id,
            frames,
            totalDuration,
          },
        };
      },
      {}
    );
  }
}
