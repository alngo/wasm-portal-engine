<<<<<<< HEAD
mod canvas;

=======
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

mod canvas;

#[wasm_bindgen]
pub fn test_fill(canvas: &mut canvas::Canvas ) {
    for x in 0..canvas.width {
        for y in 0..canvas.height {
            canvas.put_pixel(0xFF_FF_00_FF, x, y);
        }
    }
}

>>>>>>> 651de87cabbb92ad5616926f84ef95e0731d2b3d
