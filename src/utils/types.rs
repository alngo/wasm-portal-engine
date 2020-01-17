extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Xy(pub f64, pub f64);
#[wasm_bindgen]
pub struct Xyz(pub f64, pub f64, pub f64);

impl Copy for Xy {}

impl Clone for Xy {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for Xyz {}

impl Clone for Xyz {
    fn clone(&self) -> Self {
        *self
    }
}
