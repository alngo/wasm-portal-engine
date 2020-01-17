use crate::sector::Sector;
use crate::player::Player;
use crate::utils::vector::Vertex;
use serde_json::{Value, Error};
use crate::utils::decoder::{interface};

pub struct Map {
    pub vertexes: Vec<Vertex>,
    pub sectors: Vec<Sector>,
    pub player: Player
}

impl Map {
    pub fn load(json_string: &str) -> Result<Map, Error> {
        let decode: Value = serde_json::from_str(json_string)?;
        let vertexes: Vec<Vertex> = interface::vertexes(&decode["vertexes"]);
        let sectors: Vec<Sector> = interface::sectors(&decode["sectors"]);
        let player: Player = interface::player(&decode["player"]);
        Ok(
           Map {
                vertexes: vertexes,
                sectors: sectors,
                player: player
            }
        )
    }
}

impl Default for Map {
    fn default() -> Self {
        Self {
            vertexes: vec![],
            sectors: vec![],
            player: Player::default()
        }
    }
}

#[cfg(test)]
mod map_tests {
    use super::*;
    #[test]
    fn it_should_match_default_value() {
        let data = r#"
        {
            "vertexes": [
                [0.0, 0.0],
                [0.0, 5.0],
                [5.0, 0.0],
                [5.0, 5.0]
            ],
            "sectors": [
                {
                    "floor": 0.0,
                    "ceil": 20.0,
                    "vertexes_id": [0, 1, 2, 3],
                    "neighbors_id": [-1, -1, -1, -1]
                }
            ],
            "player": {
                "position": [0.0, 0,0, 0.0],
                "velocity": [0.0, 0.0],
                "angle": 0.0,
                "sector": 0
            }
        }"#;
        let map = Map::load(data);
        //Map::untyped_example();
    }
}
