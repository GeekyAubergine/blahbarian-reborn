import { AssetConfig } from "../../game/Asset";
import { ASSET_NAMES } from "../../game/Constants";
import aseprite from "./sand-tile.json";
// @ts-expect-error
import image from "./sand-tile.png";

export const SAND_TILE_CONFIG: AssetConfig = {
  name: ASSET_NAMES.SAND_TILE,
  asepriteFile: aseprite,
  spriteSheetFilePath: "/level/sand-tile.png",
};
