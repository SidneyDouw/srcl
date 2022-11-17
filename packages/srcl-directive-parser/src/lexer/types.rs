#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Scope {
    Begin,
    End,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Token {
    Identifier(String),
    Number(String),
    String(String),
    Boolean(String),
    QuotationMark(char),
    Brace(Scope),
    SquareBracket(Scope),
    AngleBracket(Scope),
    Parentheses(Scope),
    Comment(String),
    Dot,
    Comma,
    Colon,
    Semicolon,
    Slash,
    Backslash,
    Equals,
    Dash,
    Underscore,
    FatArrow,
    Pipe,
    QuestionMark,
    Ampersand,
    DirectiveStart,
    DirectiveEnd,
    Whitespace,
    Newline,
}

impl Token {
    pub fn inner(self) -> String {
        match self {
            Token::Identifier(x) => x,
            Token::Number(x) => x,
            Token::String(x) => x,
            Token::Boolean(x) => x,
            Token::Comment(x) => x,
            Token::QuotationMark(x) => x.into(),
            t => panic!("No inner data for token type {:?}", t),
        }
    }
}
