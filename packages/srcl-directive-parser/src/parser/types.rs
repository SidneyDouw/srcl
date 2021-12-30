use crate::lexer::types::Token;

#[derive(Debug)]
pub enum DirectiveValue {
    String(String),
    Number(String),
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

pub enum EnumValue {
    String(String),
    Number(String),
}

pub struct EnumProperty {
    key: String,
    value: Option<EnumValue>,
    directives: Vec<Directive>,
}

impl EnumProperty {
    pub fn new(key: String, value: Option<EnumValue>) -> EnumProperty {
        EnumProperty {
            key,
            value,
            directives: Vec::new(),
        }
    }
}

struct Enum {
    name: String,
    directives: Vec<Directive>,
    properties: Vec<EnumProperty>,
}

impl Enum {
    fn new(name: String) -> Enum {
        Enum {
            name,
            directives: Vec::new(),
            properties: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub enum PropertyValue {
    String(String),
    Number(String),
    // Interface(Interface),
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
pub struct Interface {
    pub name: String,
    pub directives: Vec<Directive>,
    pub properties: Vec<InterfaceProperty>,
}

impl Interface {
    pub fn new(name: String) -> Interface {
        Interface {
            name,
            directives: Vec::new(),
            properties: Vec::new(),
        }
    }
}

impl From<Token> for DirectiveValue {
    fn from(t: Token) -> Self {
        match t {
            Token::Identifier(s) => DirectiveValue::String(s),
            Token::String(s) => DirectiveValue::String(s),
            Token::Number(n) => DirectiveValue::Number(n),
            t => panic!("Token {:?} cannot be converted into DirectiveValue", t),
        }
    }
}

impl From<Token> for PropertyValue {
    fn from(t: Token) -> Self {
        match t {
            Token::Identifier(s) => PropertyValue::String(s),
            Token::String(s) => PropertyValue::String(s),
            Token::Number(n) => PropertyValue::Number(n),
            t => panic!("Token {:?} cannot be converted into DirectiveValue", t),
        }
    }
}

impl From<Token> for EnumValue {
    fn from(t: Token) -> Self {
        match t {
            Token::Identifier(s) => EnumValue::String(s),
            Token::String(s) => EnumValue::String(s),
            Token::Number(n) => EnumValue::Number(n),
            t => panic!("Token {:?} cannot be converted into DirectiveValue", t),
        }
    }
}
