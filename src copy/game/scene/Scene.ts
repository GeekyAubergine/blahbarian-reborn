import { Entity } from "../entity/Entity";
import { Event } from "../Events";
import { Game } from "../Game";

export class Scene {
  private entities: Entity[] = [];
  private entitiesToRemove: string[] = [];

  constructor() {}

  init(game: Game) {
    for (const entity of this.entities) {
      entity.init(game);
    }
  }

  update(game: Game, dt: number) {
    this.entities = this.entities.filter((entity) => {
      if (this.entitiesToRemove.includes(entity.getId())) {
        return false;
      }
      return true;
    });

    for (const entity of this.entities) {
      entity.update(game, dt);
    }
  }

  onEvent(game: Game, event: Event) {
    for (const entity of this.entities) {
      entity.onEvent(game, event);
    }
  }

  render(game: Game, now: number) {
    const renderer = game.getRenderer();
    const { width, height } = renderer.getSize();

    renderer.saveTransform();

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

  renderBackground(game: Game, now: number) {
  }

  addEntity(game: Game, entity: Entity) {
    entity.init(game);
    this.entities.push(entity);
    console.log("Added entity", entity);
  }

  removeEntity(entity: Entity) {
    this.entitiesToRemove.push(entity.getId());
  }

  getEntities() {
    return this.entities;
  }
}
