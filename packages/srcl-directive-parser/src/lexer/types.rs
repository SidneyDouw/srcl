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
            Token::String(s) => s,
            Token::Boolean(b) => b,
            Token::Comment(c) => c,
            Token::QuotationMark(q) => q.into(),
            _ => panic!("Token::inner() called on non-inner token"),
        }
    }
}
