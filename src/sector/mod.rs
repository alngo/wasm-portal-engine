use crate::utils::vector::Vec2;

pub struct Sector {
    pub floor: f32,
    pub ceil: f32,
    pub vertex: Vec<Vec2>,
    pub neighbors: Vec<u8>,
    pub npoints: u32
}

impl Sector {
    pub fn eg() {
    }
}

// ============
// TEST SECTION
// ============

#[cfg(test)]
mod sector_tests {
    use super::*;
    #[test]
    fn eg() {
        assert_eq!(1, 1);
    }
}
