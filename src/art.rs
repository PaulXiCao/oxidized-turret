use wasm_bindgen::prelude::*;

use crate::CreepKind;

#[wasm_bindgen(raw_module = "./js/Art.js")]
extern "C" {
    pub type Art;

    #[wasm_bindgen(method)]
    pub fn drawTurret(this: &Art, x: f32, y: f32, rotation: f32, size: f32, kind: i32);

    #[wasm_bindgen(method)]
    pub fn drawParticle(this: &Art, x: f32, y: f32);

    #[wasm_bindgen(method)]
    pub fn drawCannonParticle(this: &Art, x: f32, y: f32, r: f32);

    #[wasm_bindgen(method)]
    pub fn drawSniperParticle(this: &Art, x: f32, y: f32, x2: f32, y2: f32);

    #[wasm_bindgen(method)]
    pub fn drawCreep(this: &Art, x: f32, y: f32, health_percentage: f32, kind: CreepKind);

    #[wasm_bindgen(method)]
    pub fn drawMap(this: &Art, width: f32, height: f32);

    #[wasm_bindgen(method)]
    pub fn startCreepPath(this: &Art, x: f32, y: f32, time: f32);

    #[wasm_bindgen(method)]
    pub fn drawCreepPathLine(this: &Art, x: f32, y: f32);

    #[wasm_bindgen(method)]
    pub fn endCreepPath(this: &Art);

    #[wasm_bindgen(method)]
    pub fn drawCreepSpawn(this: &Art, x: f32, y: f32, size: f32);

    #[wasm_bindgen(method)]
    pub fn drawCreepGoal(this: &Art, x: f32, y: f32, size: f32);

    #[wasm_bindgen(method)]
    pub fn clear(this: &Art);
}
