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

#[allow(dead_code)]
impl Map {
    fn decode_data(data: &str) -> Result<Map, Error> {
        let decode: Value = serde_json::from_str(data)?;
        let mut map = Map::default();
        map.vertexes = decode::vertexes(&decode["vertexes"]);
        map.sectors = decode::sectors(&decode["sectors"]);
        map.player = decode::player(&decode["player"]);
        Ok(map)
    }
    pub fn load(json_string: &str) -> Map {
        match Map::decode_data(json_string) {
            Ok(map) => map,
            Err(e) => {
                println!("An error has occured in decode_data: {}", e);
                Map::default()
            }
        }
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
    fn it_should_match_default_values() {
        let map = Map::default();
        assert_eq!(map.vertexes.len(), 0);
        assert_eq!(map.sectors.len(), 0);
        assert_eq!(map.player.position.0, 0.0);
        assert_eq!(map.player.position.1, 0.0);
        assert_eq!(map.player.position.2, 0.0);
        assert_eq!(map.player.velocity.0, 0.0);
        assert_eq!(map.player.velocity.1, 0.0);
        assert_eq!(map.player.velocity.2, 0.0);
        assert_eq!(map.player.angle, 0.0);
        assert_eq!(map.player.sector, 0);
    }

    #[test]
    fn it_should_match_data_values() {
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
                "position": [1.0, 2.0, 3.0],
                "velocity": [4.0, 5.0, 6.0],
                "angle": 7.0,
                "sector": 8
            }
        }"#;
        let map = Map::load(data);
        assert_eq!(map.vertexes.len(), 4);
        assert_eq!(map.sectors.len(), 1);
        assert_eq!(map.sectors[0].floor, 0.0);
        assert_eq!(map.sectors[0].ceil, 20.0);
        assert_eq!(map.sectors[0].vertexes_id, [0, 1, 2, 3]);
        assert_eq!(map.sectors[0].neighbors_id, [-1, -1, -1, -1]);
        assert_eq!(map.player.position.0, 1.0);
        assert_eq!(map.player.position.1, 2.0);
        assert_eq!(map.player.position.2, 3.0);
        assert_eq!(map.player.velocity.0, 4.0);
        assert_eq!(map.player.velocity.1, 5.0);
        assert_eq!(map.player.velocity.2, 6.0);
        assert_eq!(map.player.angle, 7.0);
        assert_eq!(map.player.sector, 8);
    }
}
