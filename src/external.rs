use crate::recycled_list::RecycledListRef;
use crate::utils::FloatPosition;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct ExternalTurret {
    pub pos: FloatPosition,
    pub rotation: f32, // orientation/angle in RAD
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct TurretRef {
    pub turret: ExternalTurret,
    pub turret_ref: RecycledListRef,
}
