use crate::sector::Sector;
use crate::player::Player;
use serde_json::{Value, Error};

pub struct Vertex(f64, f64);

pub struct Map {
    pub vertexes: Vec<Vertex>,
    pub sectors: Vec<Sector>,
    pub player: Player
}

pub fn decode_vertexes(array: &serde_json::Value) -> Vec<Vertex> {
    let mut vertexes = vec![];
    let decomposed = array.as_array().unwrap();
    for vec in decomposed {
        let v = vec.as_array().unwrap();
        let x = v[0].as_f64().unwrap();
        let y = v[1].as_f64().unwrap();
        vertexes.push(Vertex(x, y));
    }
    vertexes
}

pub fn decode_sectors(array: &serde_json::Value) -> Vec<Sector> {
    let mut sectors = vec![];
    sectors
}

pub fn decode_player(object: &serde_json::Value) -> Player {
    Player::default()
}

impl Map {
    pub fn load(json_string: &str) -> Result<Map, Error> {
        let decode: Value = serde_json::from_str(json_string)?;
        let vertexes: Vec<Vertex> = decode_vertexes(&decode["vertexes"]);
        let sectors: Vec<Sector> = decode_sectors(&decode["sectors"]);
        let player: Player = decode_player(&decode["player"]);
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
                {"floor": 0.0, "ceil": 20.0, "vertexes": [0, 1, 2, 3]}
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
