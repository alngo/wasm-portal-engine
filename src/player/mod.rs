extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use crate::utils::vector::Vec3;
use crate::utils::vector::Vec2;

#[wasm_bindgen]
pub struct Player {
    pub position: Vec3,
    pub velocity: Vec3,
    pub angle: f64,
    pub sector: u64
}

#[wasm_bindgen]
impl Player {
    pub fn move_player(&mut self, dx: f64, dy: f64) {
        let _px = self.position.x;
        let _py = self.position.y;

        self.position.x += dx;
        self.position.y += dy;
    }
    pub fn move_control(&mut self,
        forward: bool,
        backward: bool,
        left: bool,
        right: bool
    ) -> Vec2 {
        let mut vec2 = Vec2::default();
        if forward {
            vec2.x += self.angle.cos() * 0.2;
            vec2.y += self.angle.sin() * 0.2;
        }
        if backward {
            vec2.x -= self.angle.cos() * 0.2;
            vec2.y -= self.angle.sin() * 0.2;
        }
        if left {
            vec2.x += self.angle.sin() * 0.2;
            vec2.y -= self.angle.cos() * 0.2;
        }
        if right {
            vec2.x -= self.angle.sin() * 0.2;
            vec2.y += self.angle.cos() * 0.2;
        }
        let push = forward || backward || left || right;
        let acc: f64 = if push { 0.4 } else { 0.2 };
        self.velocity.x = self.velocity.x * (1.0 - acc) + vec2.x * acc;
        self.velocity.y = self.velocity.y * (1.0 - acc) + vec2.y * acc;
        vec2
    }
}

impl Default for Player {
    fn default() -> Self {
        Self {
            position: Vec3::default(),
            velocity: Vec3::default(),
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
        assert_eq!(player.position.x, 5.0);
        assert_eq!(player.position.y, 5.0);
    }

    #[test]
    fn it_should_move_velocity() {
        let mut player = Player::default();
        player.move_control(true, false, true, false);
        assert_ne!(player.velocity.x, 0.0);
        assert_ne!(player.velocity.y, 0.0);
    }
}
