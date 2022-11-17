// #[derive(Debug, PartialEq, Eq, Hash, Clone)]
#[derive(Debug, Clone)]
pub enum Scope {
    Begin,
    End,
}

// #[derive(Debug, PartialEq, Eq, Hash, Clone)]
#[derive(Debug, Clone)]
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
    Star,
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
    pub(crate) fn inner(self) -> String {
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

    pub(crate) fn kind(&self) -> String {
        match self {
            Token::Identifier(_) => "Identifier".into(),
            Token::Number(_) => "Number".into(),
            Token::String(_) => "String".into(),
            Token::Boolean(_) => "Boolean".into(),
            Token::QuotationMark(_) => "QuotationMark".into(),
            Token::Brace(_) => "Brace".into(),
            Token::SquareBracket(_) => "SquareBracket".into(),
            Token::AngleBracket(_) => "AngleBracket".into(),
            Token::Parentheses(_) => "Parentheses".into(),
            Token::Comment(_) => "Comment".into(),
            Token::Dot => "Dot".into(),
            Token::Comma => "Comma".into(),
            Token::Colon => "Colon".into(),
            Token::Semicolon => "Semicolon".into(),
            Token::Slash => "Slash".into(),
            Token::Star => "Star".into(),
            Token::Backslash => "Backslash".into(),
            Token::Equals => "Equals".into(),
            Token::Dash => "Dash".into(),
            Token::Underscore => "Underscore".into(),
            Token::FatArrow => "FatArrow".into(),
            Token::Pipe => "Pipe".into(),
            Token::QuestionMark => "QuestionMark".into(),
            Token::Ampersand => "Ampersand".into(),
            Token::DirectiveStart => "DirectiveStart".into(),
            Token::DirectiveEnd => "DirectiveEnd".into(),
            Token::Whitespace => "Whitespace".into(),
            Token::Newline => "Newline".into(),
        }
    }
}
