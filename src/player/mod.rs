extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use crate::utils::coordinate::Coordinate;

#[wasm_bindgen]
pub struct Player {
    pub position: Coordinate,
    pub velocity: Coordinate,
    pub angle: f32,
    pub sector: u32
}

#[wasm_bindgen]
impl Player {
    pub fn move_player(&mut self, dx: f32, dy: f32) {
        let px = self.position.x;
        let py = self.position.y;

        self.position.x += dx;
        self.position.y += dy;
    }
}

impl Default for Player {
    fn default() -> Self {
        Self {
            position: Coordinate::default(),
            velocity: Coordinate::default(),
            angle: 0.0,
            sector: 0
        }
    }
}

// ============
// TEST SECTION
// ============

#[cfg(test)]
mod player_tests {
    use super::*;
    #[test]
    fn it_should_move() {
        let mut player = Player::default();
        player.move_player(5.0, 5.0);
        assert_eq!(player.position.x, 5.0);
        assert_eq!(player.position.y, 5.0);
    }
}
