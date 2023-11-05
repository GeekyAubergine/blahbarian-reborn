import { POT_1_CONFIG } from "../assets/level/pot-1";
import { SAND_TILE_CONFIG } from "../assets/level/sand-tile";
import { AsepriteFile } from "../engine/SpriteSheet";

export type AssetConfig = {
    name: string;
    asepriteFile: AsepriteFile;
    spriteSheetFilePath: string,
}

export const ASSETS = [
    POT_1_CONFIG,
    SAND_TILE_CONFIG,
]