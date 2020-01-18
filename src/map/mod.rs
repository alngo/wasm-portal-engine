use crate::sector::Sector;
use crate::player::Player;
use crate::utils::types::Xy;
use serde_json::{Value, Error};
use crate::utils::decode;

pub struct Map {
    pub vertexes: Vec<Xy>,
    pub sectors: Vec<Sector>,
    pub player: Player
}

impl Map {
    #[allow(dead_code)]
    pub fn load(json_string: &str) -> Result<Map, Error> {
        let decode: Value = serde_json::from_str(json_string)?;
        let mut map = Map::default();
        map.vertexes = decode::vertexes(&decode["vertexes"]);
        map.sectors = decode::sectors(&decode["sectors"]);
        map.player = decode::player(&decode["player"]);
        Ok(map)
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
                "position": [1.0, 2,0, 0.0],
                "velocity": [0.0, 0.0, 0.0],
                "angle": 0.0,
                "sector": 0
            }
        }"#;
        let _map = Map::load(data);
        //Map::untyped_example();
    }
}
