extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Copy for Vec3 {}

impl Clone for Vec3 {
    fn clone(&self) -> Self {
        *self
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

#[wasm_bindgen]
pub struct Vec2 {
    pub x: f64,
    pub y: f64
}

impl Copy for Vec2 {}

impl Clone for Vec2 {
    fn clone(&self) -> Self {
        *self
    }
}

impl Default for Vec2 {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0
        }
    }
}

pub struct Vertex(pub f64, pub f64);

// ============
// TEST SECTION
// ============

#[cfg(test)]
mod vec3_tests {
    use super::*;
    #[test]
    fn it_should_match_0_vec3() {
        let coord = Vec3::default();
        assert_eq!(coord.x, 0.0);
        assert_eq!(coord.y, 0.0);
        assert_eq!(coord.z, 0.0);
    }
}

#[cfg(test)]
mod vec2_tests {
    use super::*;
    #[test]
    fn it_should_match_0_vec2() {
        let coord = Vec2::default();
        assert_eq!(coord.x, 0.0);
        assert_eq!(coord.y, 0.0);
    }
}
