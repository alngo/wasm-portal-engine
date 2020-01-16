use crate::utils::vector::Vec2;
use crate::sector::Sector;

pub struct Map {
    pub vertexes: Vec<Vec2>,
    pub sectors: Vec<Sector>,
}

impl Default for Map {
    fn default() -> Self {
        Self {
            vertexes: vec![
                Vec2(0.0, 0.0),
                Vec2(5.0, 0.0),
                Vec2(5.0, 5.0),
                Vec2(0.0, 5.0)
            ]
            sectors: vec![Sector {
                floor: 0.0,
                ceil: 20.0,
                vertex: vec![0,1,2,3],
                neighbors vec![]
            }]
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
