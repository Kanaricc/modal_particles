use std::io::Write;

use wasm_bindgen::prelude::*;
use modal_particles::{Encoder, Decoder};

#[wasm_bindgen]
pub fn encode(str: &str) -> String {
    let mut encoder = Encoder::new(Vec::new());
    encoder.write_all(str.as_bytes()).unwrap();

    String::from_utf8(encoder.get_writer()).unwrap()
}

#[wasm_bindgen]
pub fn decode(str: &str) -> String {
    let mut decoder = Decoder::new(Vec::new());
    decoder.write_all(str.as_bytes()).unwrap();

    String::from_utf8(decoder.get_writer()).unwrap()
}