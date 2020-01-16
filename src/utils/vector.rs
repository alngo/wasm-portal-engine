extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
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
pub struct Vec2(f32, f32);

impl Copy for Vec2 {}

impl Clone for Vec2 {
    fn clone(&self) -> Self {
        *self
    }
}

impl Default for Vec2 {
    fn default() -> Self {
        Self(0.0, 0.0)
    }
}

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
        let (x, y) = coord;
        assert_eq!(x, 0.0);
        assert_eq!(y, 0.0);
    }
}
