#![allow(dead_code)]

pub mod lexer;
pub mod parser;
pub mod transformer;

use parser::Parser;
use transformer::to_js_obj;
use wasm_bindgen::prelude::*;

pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn transform_file(code: &str) -> Result<String, JsValue> {
    set_panic_hook();

    let mut parser = Parser::new(code);

    // TODO: Add line and column to error messages, maybe even make it pretty
    let interface = parser.parse()?;

    let s = to_js_obj(interface);

    let output = code.replace(
        "export default Component",
        format!("{}\n\nexport default Component", s).as_str(),
    );

    Ok(output)
}
