extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    buffer: Vec<u32>,
}

#[wasm_bindgen]
impl Canvas {
    pub fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        self.buffer.resize(self.width * self.height, 0);
    }
    pub fn put_pixel(&mut self, color: u32, x: usize, y: usize) {
        if (y * self.width + x) < (self.width * self.height) {
            self.buffer[y * self.width + x] = color;
        }
    }
}

#[wasm_bindgen]
pub fn create_canvas(width: usize, height: usize) -> Canvas {
    let canvas = Canvas {width: width, height: height, buffer: vec![0; width * height]};
    canvas
}

// ===========
// TEST SECTOR
// ===========

#[cfg(test)]
mod canvas_tests {
    use super::*;
    #[test]
    fn it_should_return_a_50_50_canvas() {
        let canvas = create_canvas(50, 50);
        assert_eq!(canvas.buffer.len(), 50 * 50);
    }

    #[test]
    fn it_should_resize_to_10_10_canvas() {
        let mut canvas = create_canvas(50, 50);
        assert_eq!(canvas.buffer.len(), 50 * 50);
        canvas.resize(10, 10);
        assert_eq!(canvas.buffer.len(), 10 * 10);
    }

    #[test]
    fn it_should_have_a_colored_pixel() {
        let mut canvas = create_canvas(10, 10);
        assert_eq!(canvas.buffer.len(), 10 * 10);
        canvas.put_pixel(0xFF_FF_00_FF, 0, 0);
        assert_eq!(canvas.buffer[0], 0xFF_FF_00_FF);
    }
}
