use crate::sector::Sector;
use crate::player::Player;
use crate::utils::vector::Vertex;
use crate::utils::types::{Xy, Xyz};
use crate::serde_json::{Value, Map};

mod decoder {
    pub fn array_to_vector(array: &super::Value) -> &Vec<super::Value> {
        array.as_array().unwrap()
    }
    pub fn object_to_map(array: &super::Value) -> &super::Map<String, super::Value> {
        array.as_object().unwrap()
    }
    pub fn array2_to_xy(array: &super::Value) -> super::Xy {
        let a = array.as_array().unwrap();
        super::Xy(value_to_f64(&a[0]), value_to_f64(&a[1]))
    }
    pub fn array3_to_xyz(array: &super::Value) -> super::Xyz {
        let a = array.as_array().unwrap();
        super::Xyz(value_to_f64(&a[0]), value_to_f64(&a[1]), value_to_f64(&a[2]))
    }
    pub fn array_to_f64_vector(array: &super::Value) -> Vec<f64> {
        let mut vector = array_to_vector(array);
        let vec = vec![];
        for val in vector {
            vec.push(value_to_f64(val))
        }
        vec
    }
    pub fn array_to_i64_vector(array: &super::Value) -> Vec<i64> {
        let mut vector = array_to_vector(array);
        let vec = vec![];
        for val in vector {
            vec.push(value_to_i64(val))
        }
        vec
    }
    pub fn array_to_u64_vector(array: &super::Value) -> Vec<u64> {
        let mut vector = array_to_vector(array);
        let vec = vec![];
        for val in vector {
            vec.push(value_to_u64(val))
        }
        vec
    }
    pub fn value_to_f64(val: &super::Value) -> f64 {
        val.as_f64().unwrap()
    }
    pub fn value_to_u64(val: &super::Value) -> u64 {
        val.as_u64().unwrap()
    }
    pub fn value_to_i64(val: &super::Value) -> i64 {
        val.as_i64().unwrap()
    }
}

pub mod interface {
    pub fn vertexes(array: &serde_json::Value) -> Vec<super::Vertex> {
        let mut vertexes = vec![];
        for data in super::decoder::array_to_vector(array) {
            vertexes.push(super::decoder::array2_to_xy(data));
        }
        vertexes
    }
    pub fn sectors(array: &serde_json::Value) -> Vec<super::Sector> {
        let mut sectors = vec![];
        let decomposed = super::decoder::array_to_vector(array);
        for data in decomposed {
            let mut sector = super::Sector::default();
            sector.floor = super::decoder::value_to_f64(&data["floor"]);
            sector.ceil = super::decoder::value_to_f64(&data["ceil"]);
            sector.vertexes_id = super::decoder::array_to_u64_vector(&data["vertexes_id"]);
            sector.neighbors_id = super::decoder::array_to_i64_vector(&data["neighbors_id"]);
            sectors.push(sector);
        }
        sectors
    }
    pub fn player(object: &serde_json::Value) -> super::Player {
        let mut player = super::Player::default();
        let data = super::decoder::object_to_map(object);
        player.position = super::decoder::array3_to_xyz(&data["position"]);
        player.velocity = super::decoder::array2_to_xy(&data["velocity"]);
        player.angle = super::decoder::value_to_f64(&data["angle"]);
        player.sector = super::decoder::value_to_u64(&data["sector"]);
        player
    }

}
