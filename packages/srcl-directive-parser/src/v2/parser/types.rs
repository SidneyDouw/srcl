use crate::v2::lexer::tokens::Token;

#[derive(Debug)]
pub struct Interface {
    pub name: String,
    pub properties: Vec<InterfaceProperty>,
}

impl Interface {
    pub fn new() -> Interface {
        Interface {
            name: String::new(),
            properties: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct InterfaceProperty {
    pub key: String,
    pub value: PropertyValue,
    pub directives: Vec<Directive>,
}

impl InterfaceProperty {
    pub fn new(key: String, value: PropertyValue) -> InterfaceProperty {
        InterfaceProperty {
            key,
            value,
            directives: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub enum PropertyValue {
    String(String),
    Number(String),
    Boolean(String),
    // TODO: Nested Interfaces / Objects
    // Interface(Interface),
}

#[derive(Debug)]
pub struct Directive {
    pub key: String,
    pub value: DirectiveValue,
}

impl Directive {
    pub fn new(key: String, value: DirectiveValue) -> Directive {
        Directive { key, value }
    }
}

#[derive(Debug)]
pub enum DirectiveValue {
    String(String),
    Number(String),
    Boolean(String),
}

impl From<Token> for DirectiveValue {
    fn from(t: Token) -> Self {
        match t {
            Token::String(s) => DirectiveValue::String(format!("'{}'", s)),
            Token::Number(n) => DirectiveValue::Number(n),
            Token::Boolean(b) => DirectiveValue::Boolean(b),
            t => panic!(
                "Token of kind {} cannot be converted into DirectiveValue",
                t.kind()
            ),
        }
    }
}

impl From<Token> for PropertyValue {
    fn from(t: Token) -> Self {
        match t {
            Token::Identifier(s) => PropertyValue::String(s),
            Token::String(s) => {
                if s.contains('\'') {
                    PropertyValue::String(format!("\"{}\"", s))
                } else {
                    PropertyValue::String(format!("'{}'", s))
                }
            }
            Token::Number(n) => PropertyValue::Number(n),
            Token::Boolean(b) => PropertyValue::Boolean(b),
            t => panic!(
                "Token of kind {} cannot be converted into PropertyValue",
                t.kind()
            ),
        }
    }
}

// impl From<Vec<Token>> for PropertyValue {
//     fn from(v: Vec<Token>) -> Self {
//         let mut out = v
//             .into_iter()
//             .map(|t| match t {
//                 Token::Identifier(s) => s,
//                 Token::String(s) => format!("\"{}\"", s).into(),
//                 Token::Number(n) => n,
//                 t => panic!(
//                     "Token {:?} in Union vector cannot be converted into PropertyValue",
//                     t
//                 ),
//             })
//             .collect::<Vec<_>>()
//             .join(", ");

//         out = format!("[{}]", out).into();

//         PropertyValue::Expression(out)
//     }
// }
