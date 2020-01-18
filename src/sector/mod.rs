pub struct Sector {
    pub floor: f64,
    pub ceil: f64,
    pub vertexes_id: Vec<u64>,
    pub neighbors_id: Vec<i64>,
}

impl Default for Sector {
    fn default() -> Self {
        Self {
            floor: 0.0,
            ceil: 20.0,
            vertexes_id: Vec::new(),
            neighbors_id: Vec::new(),
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
