import { SEA_GRASS_CONFIG } from "../../../assets/plants/seagrass";
import { Vector } from "../../../engine/Vector";
import { ENTITY_NAMES } from "../../Constants";
import { Plant } from "./Plant";

export class SeaGrass extends Plant {
  constructor(position: Vector) {
    super(
      ENTITY_NAMES.SEA_GRASS,
      position,
      0,
      new Vector(0, 0),
      0,
      SEA_GRASS_CONFIG,
      "seagrass-idle",
      "seagrass-active"
    );
  }
}
