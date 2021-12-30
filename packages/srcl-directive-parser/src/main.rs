use srcl_directive_parser::transform_file;
use std::env;

fn main() -> Result<(), String> {
    let args = env::args().collect::<Vec<String>>();

    let filepath = args
        .get(1)
        .ok_or_else(|| "Please specify a filepath as the first parameter".to_owned())?;

    let code = std::fs::read_to_string(filepath)
        .map_err(|e| format!("Error reading file at \"{}\"\n{}", filepath, e))
        .unwrap();

    let output = transform_file(&code).unwrap();

    println!("{}", output);

    Ok(())
}
