use std::fs;
use crate::sector::Sector;
use crate::player::Player;
use serde_json::{Result, Value};

pub struct Vertex(f32, f32);

pub struct Map {
    pub vertexes: Vec<Vertex>,
    pub sectors: Vec<Sector>,
    pub player: Player
}

impl Map {
    pub fn load(&mut self, filename: &str) -> Map {
        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");
        let decode = serde_json::from_str(&contents).unwrap();
        print!("deserialized: {:?}", decode);
        Map {
            vertexes: decode.vertexes,
            sectors: decode.sectors,
        }
    }
}

#[cfg(test)]
mod map_tests {
    use super::*;
    #[test]
    fn it_should_match_default_value() {
    }
}
