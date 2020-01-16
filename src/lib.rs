extern crate more_asserts;
extern crate wasm_bindgen;
extern crate serde_json;
extern crate sdl2;
use wasm_bindgen::prelude::*;

mod utils;
mod map;
mod sector;
mod canvas;
mod player;

#[wasm_bindgen]
pub fn test_fill(canvas: &mut canvas::Canvas ) {
    for x in 0..canvas.width {
        for y in 0..canvas.height {
            canvas.put_pixel(0xFF_FF_00_FF, x, y);
        }
    }
}
