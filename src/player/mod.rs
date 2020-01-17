extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use crate::utils::types::{Xy, Xyz};

#[wasm_bindgen]
pub struct Player {
    pub position: Xyz,
    pub velocity: Xyz,
    pub angle: f64,
    pub sector: u64 }

#[wasm_bindgen]
impl Player {
    pub fn move_player(&mut self, dx: f64, dy: f64) {
        let Xyz(mut _px, mut _py, _) = self.position;

        self.position.0 += dx;
        self.position.1 += dy;
    }
    pub fn move_control(&mut self,
        forward: bool,
        backward: bool,
        left: bool,
        right: bool
    ) -> Xy {
        let mut dir = Xy(0.0, 0.0);
        if forward {
            dir.0 += self.angle.cos() * 0.2;
            dir.1 += self.angle.sin() * 0.2;
        }
        if backward {
            dir.0 -= self.angle.cos() * 0.2;
            dir.1 -= self.angle.sin() * 0.2;
        }
        if left {
            dir.0 += self.angle.sin() * 0.2;
            dir.1 -= self.angle.cos() * 0.2;
        }
        if right {
            dir.0 -= self.angle.sin() * 0.2;
            dir.1 += self.angle.cos() * 0.2;
        }
        let push = forward || backward || left || right;
        let acc: f64 = if push { 0.4 } else { 0.2 };
        self.velocity.0 = self.velocity.0 * (1.0 - acc) + dir.0 * acc;
        self.velocity.1 = self.velocity.1 * (1.0 - acc) + dir.1 * acc;
        dir
    }
}

impl Default for Player {
    fn default() -> Self {
        Self {
            position: Xyz(0.0, 0.0, 0.0),
            velocity: Xyz(0.0, 0.0, 0.0),
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
    fn it_should_move_position() {
        let mut player = Player::default();
        player.move_player(5.0, 5.0);
        assert_eq!(player.position.0, 5.0);
        assert_eq!(player.position.1, 5.0);
    }

    #[test]
    fn it_should_move_velocity() {
        let mut player = Player::default();
        player.move_control(true, false, true, false);
        assert_ne!(player.velocity.0, 0.0);
        assert_ne!(player.velocity.1, 0.0);
    }
}
