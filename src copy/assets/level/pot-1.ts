import { AssetConfig } from "../../game/Asset";
import { ASSET_NAMES } from "../../game/Constants";
import aseprite from "./pot-1.json";
// @ts-expect-error
import image from "./pot-1.png";

export const POT_1_CONFIG: AssetConfig = {
  name: ASSET_NAMES.POT_1,
  asepriteFile: aseprite,
  spriteSheetFilePath: "/level/pot-1.png",
};
