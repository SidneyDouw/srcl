use srcl_directive_parser::transform_file;
use std::env;

fn main() -> Result<(), String> {
    let args = env::args().collect::<Vec<String>>();

    let filepath = args
        .get(1)
        .ok_or_else(|| "Please specify a filepath as the first parameter".to_owned())?;

    let output = transform_file(filepath);

    println!("{}", output);

    Ok(())
}
