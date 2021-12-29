use srcl_directive_parser::lexer::Lexer;
use srcl_directive_parser::parser::Parser;
use srcl_directive_parser::transformer::to_js_obj;

#[test]
fn can_lexer_lex() {
    let input = std::fs::read_to_string("tests/fixtures/src.tsx").expect("Could not read file");
    let mut lexer = Lexer::new(&input);
    while let Some(_c) = lexer.next() {}
}

#[test]
fn can_parser_parse() {
    let input = std::fs::read_to_string("tests/fixtures/src.tsx").expect("Could not read file");

    let mut parser = Parser::new(&input);
    let interface = parser.parse();

    println!("{:?}", interface.expect("Could not parse"));
}

#[test]
fn can_transformer_transform() {
    let input = std::fs::read_to_string("tests/fixtures/src.tsx").expect("Could not read file");

    let mut parser = Parser::new(&input);
    let interface = parser.parse().expect("Could not parse");

    let s = to_js_obj(interface);

    println!(
        "{}",
        input.replace(
            "export default Component",
            format!("{}\n\nexport default Component", s).as_str()
        )
    );
}
