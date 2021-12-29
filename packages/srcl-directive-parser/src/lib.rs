#![allow(dead_code)]

pub mod lexer;
pub mod parser;
pub mod transformer;

use parser::Parser;
use transformer::to_js_obj;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn transform_file(filepath: &str) -> String {
    let input = std::fs::read_to_string(filepath)
        .map_err(|e| format!("Error reading file at \"{}\"\n{}", filepath, e))
        .unwrap();

    let mut parser = Parser::new(&input);

    // TODO: Add line and column to error messages, maybe even make it pretty
    let interface = parser.parse().unwrap();

    let s = to_js_obj(interface);

    let output = input.replace(
        "export default Component",
        format!("{}\n\nexport default Component", s).as_str(),
    );

    output
}
