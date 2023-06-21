extern crate nalgebra as na;
extern crate pythagore as py;

mod cell;
mod universe;
mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello world!");
}
