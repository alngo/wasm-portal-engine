pub struct Sector {
    pub floor: f32,
    pub ceil: f32,
    pub vertex: Vec<u32>,
    pub neighbors: Vec<u8>,
}

impl Default for Sector {
    fn default() -> Self {
        Self {
            floor: 0.0,
            ceil: 20.0,
            vertex: Vec::new(),
            neighbors: Vec::new(),
        }
    }
}

// ============
// TEST SECTION
// ============

#[cfg(test)]
mod sector_tests {
    use super::*;
    #[test]
    fn it_should_match_default_value() {
        let sector = Sector::default();
        assert_eq!(sector.floor, 0.0);
        assert_eq!(sector.ceil, 20.0);
    }
}
