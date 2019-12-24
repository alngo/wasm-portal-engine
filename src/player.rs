extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Pos {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[wasm_bindgen]
pub struct Player {
    pos: Pos,
    pub angle: f32,
    pub sector: u32,
}

#[wasm_bindgen]
impl Player {
    pub fn pmove() {
    }
    pub fn protate() {
    }
    pub fn psector() {
    }
}
