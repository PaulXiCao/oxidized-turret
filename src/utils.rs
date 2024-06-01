use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct GridPosition {
    pub x: u32,
    pub y: u32,
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct FloatPosition {
    pub x: f32,
    pub y: f32,
}

pub fn distance(a: FloatPosition, b: FloatPosition) -> f32 {
    let dx = b.x - a.x;
    let dy = b.y - a.y;
    (dx.powi(2) + dy.powi(2)).sqrt()
}

pub fn to_float_position(pos: GridPosition, factor: f32) -> FloatPosition {
    FloatPosition {
        x: pos.x as f32 * factor,
        y: pos.y as f32 * factor,
    }
}

pub fn to_creep_position(pos: GridPosition, factor: f32) -> FloatPosition {
    FloatPosition {
        x: (pos.x as f32 + 0.5) * factor,
        y: (pos.y as f32 + 0.5) * factor,
    }
}

pub fn to_grid_position(pos: FloatPosition, factor: f32) -> GridPosition {
    GridPosition {
        x: (pos.x / factor).floor() as u32,
        y: (pos.y / factor).floor() as u32,
    }
}

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
