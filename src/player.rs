extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[wasm_bindgen]
pub struct Player {
    pub position: Position,
    pub angle: f32,
    pub velocity: u32,

}

#[wasm_bindgen]
impl Player {
    pub fn moveForward() {
    }
    pub fn moveBackward() {
    }
    pub fn moveLeft() {
    }
    pub fn moveRight() {
    }
    pub fn rotateLeft() {
    }
    pub fn rotateRight() {
    }
}
