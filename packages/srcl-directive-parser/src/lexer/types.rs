#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Scope {
    Begin,
    End,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Token {
    Identifier(String),
    Number(String),
    Brace(Scope),
    SquareBracket(Scope),
    AngleBracket(Scope),
    Parentheses(Scope),
    Comment(String),
    QuotationMark(char),
    Dot,
    Comma,
    Colon,
    Semicolon,
    Slash,
    Equals,
    FatArrow,
    Pipe,
    QuestionMark,
    DirectiveStart,
    DirectiveEnd,
    Whitespace,
    Newline,
}

impl Token {
    pub fn inner(self) -> String {
        match self {
            Token::Identifier(s) => s,
            Token::Number(n) => n,
            _ => panic!("Token::inner() called on non-inner token"),
        }
    }
}
