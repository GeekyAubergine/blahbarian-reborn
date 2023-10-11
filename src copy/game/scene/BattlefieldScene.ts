import { Sprite, SpriteSheet } from "../../engine/SpriteSheet";
import { Vector } from "../../engine/Vector";
import { SeaGrass } from "../entity/plants/SeaGrass";
import { Game } from "../Game";
import { Scene } from "./Scene";

const WIDTH = 9;
const HEIGHT = 5;
const TILE_SIZE = 32;

export class BattlefieldScene extends Scene {
  potSprite: Sprite | null = null;

  constructor() {
    super();
  }

  init(game: Game): void {
    super.init(game);

    this.potSprite =
      game.getRenderer().findSpriteSheet("pot-1")?.findFrame(0) ?? null;

    console.log("BattlefieldScene.init", this.potSprite);

    for (let x = 0; x < WIDTH; x++) {
      for (let y = 0; y < HEIGHT; y++) {
        this.addEntity(
          game,
          new SeaGrass(new Vector(x * TILE_SIZE, y * TILE_SIZE))
        );
      }
    }
  }

  render(game: Game, now: number) {
    const renderer = game.getRenderer();

    renderer.saveTransform();

    renderer.translate(TILE_SIZE, TILE_SIZE)

    renderer.scale(game.getCamera().getScale(), game.getCamera().getScale());

    renderer.translate(
      Math.floor(-game.getCamera().getPosition().x),
      Math.floor(-game.getCamera().getPosition().y)
    );

    this.renderBackground(game, now);

    for (const entity of this.getEntities()) {
      entity.render(renderer, now);
    }

    renderer.restoreTransform();
  }

  renderBackground(game: Game, now: number): void {
    for (let x = 0; x < WIDTH; x++) {
      for (let y = 0; y < HEIGHT; y++) {
        game
        .getRenderer()
        .renderSpriteSheetFrame("sand-tile", (x + y) % 2, new Vector(x * TILE_SIZE, y * TILE_SIZE), 0);
        game
          .getRenderer()
          .renderSpriteSheetFrame("pot-1", 0, new Vector(x * TILE_SIZE, y * TILE_SIZE), 0);
      }
    }
  }
}
