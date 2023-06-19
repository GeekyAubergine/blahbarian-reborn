export const RENDER_COLLIDERS = true;
export const SPRITE_SCALE = 4;

export const ENTITY_NAMES = {
  SEA_GRASS: "seagrass",
};

export const ASSET_NAMES = {
  POT_1: "pot-1",
  SAND_TILE: "sand-tile",
};

export type ENTITY_NAMES = typeof ENTITY_NAMES[keyof typeof ENTITY_NAMES];
export type ASSET_NAMES = typeof ASSET_NAMES[keyof typeof ASSET_NAMES];

export enum PowerupEffectType {
  ADD_HEALTH = "ADD_HEALTH",
}

export type PowerupEffectBase<T extends PowerupEffectType, D> = {
  type: T;
  data: D;
};

export type PowerupEffectAddHealth = PowerupEffectBase<
  PowerupEffectType.ADD_HEALTH,
  { amount: number }
>;

export type PowerupEffect = PowerupEffectAddHealth;

export const POWERUP_EFFECTS: Record<string, PowerupEffect> = {};
