import { ENTITY_NAMES } from "../../game/Constants";
import { EntityConfig } from "../../game/entity/Entity";
import aseprite from "./seagrass.json";
// @ts-expect-error
import image from "./seagrass.png";

console.log(aseprite, image);

export const SEA_GRASS_CONFIG: EntityConfig = {
  name: ENTITY_NAMES.SEA_GRASS,
  asepriteFile: aseprite,
  spriteSheetFilePath: '/plants/seagrass.png',
  attributes: {},
};
