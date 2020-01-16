use crate::sector::Sector;
use crate::player::Player;
use serde_json::{Value};

pub struct Vertex(f32, f32);

pub struct Map {
    pub vertexes: Vec<Vertex>,
    pub sectors: Vec<Sector>,
    pub player: Player
}

impl Map {
    pub fn load(json_string: &str) {
        let decode: Value = serde_json::from_str(&json_string)?;
        print!("deserialized: {:?}", decode);
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
            {0.0, 0.0},
            {0.0, 5.0},
            {5.0, 0.0},
            {5.0, 5.0}
        ],
        "sectors": [
            {"floor": 0.0, "ceil": 20.0, "vertexes": [0, 1, 2, 3]}
        ],
        "player": {
            "position": {
                "x": 0.0,
                "y": 0.0,
                "z": 0.0
            },
            "velocity": {
                "x": 0.0,
                "y": 0.0
            },
            "angle": 0.0,
            "sector": 0
        }
    }"#;
        Map::load(data);
    }
}
