extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Coordinate {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Copy for Coordinate {}

impl Clone for Coordinate {
    fn clone(&self) -> Self {
        *self
    }
}

impl Default for Coordinate {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

// ============
// TEST SECTION
// ============

#[cfg(test)]
mod coordinate_tests {
    use super::*;
    #[test]
    fn it_should_match_0() {
        let mut coord = Coordinate::default();
        assert_eq!(coord.x, 0.0);
        assert_eq!(coord.y, 0.0);
        assert_eq!(coord.z, 0.0);
    }
}
