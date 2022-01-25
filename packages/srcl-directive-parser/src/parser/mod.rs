pub mod types;

use crate::lexer::types::{Scope, Token};
use crate::lexer::{Lexer, LexerContext};

use self::types::{Directive, EnumProperty, Interface, InterfaceProperty};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl Parser<'_> {
    pub fn new(string: &str) -> Parser<'_> {
        Parser {
            lexer: Lexer::new(string),
        }
    }

    pub fn parse(&mut self) -> Result<Interface, String> {
        let mut interface = Interface::new("".into());

        while let Some(token) = self.lexer.peek() {
            enum ResultType {
                Directive(Vec<Directive>),
                Interface((String, Vec<InterfaceProperty>)),
            }

            let result: Result<ResultType, String> = match token {
                Token::DirectiveStart => Ok(ResultType::Directive(self.parse_directive()?)),
                Token::Identifier(s) => match s.as_str() {
                    "interface" => Ok(ResultType::Interface(self.parse_interface()?)),
                    // "type" => self.parse_type(),
                    _ => {
                        self.ignore();
                        continue;
                    }
                },
                _ => {
                    self.ignore();
                    continue;
                }
            };

            //TODO: Cannot handle multiple interfaces yet

            match result? {
                ResultType::Directive(directives) => {
                    interface.directives.extend(directives);
                }
                ResultType::Interface((name, properties)) => {
                    interface.name = name;
                    interface.properties = properties;
                }
            }
        }

        Ok(interface)
    }

    fn parse_enum(&mut self) -> Result<Vec<EnumProperty>, String> {
        self.identifier()?;
        self.identifier()?.inner();
        self.brace(Scope::Begin)?;

        let mut properties: Vec<EnumProperty> = Vec::new();

        loop {
            if let Some(token) = self.lexer.peek() {
                let key_val = match token {
                    Token::Identifier(_) => self.enum_entry(),
                    Token::Comment(_) => {
                        self.parse_comment()?;
                        continue;
                    }
                    Token::Brace(Scope::End) => {
                        self.brace(Scope::End)?;
                        break;
                    }
                    _ => Err(format!(
                        "Unexpected token: {:?} at {:?}",
                        token,
                        self.lexer.get_position()
                    )),
                }?;

                properties.push(EnumProperty::new(key_val.0, key_val.1.map(|x| x.into())));
            } else {
                return Err("Expected Closing Brace".into());
            }
        }

        Ok(properties)
    }

    fn enum_entry(&mut self) -> Result<(String, Option<Token>), String> {
        let key = self.identifier()?.inner();
        let mut value = None;

        if let Some(Token::Equals) = self.lexer.peek() {
            self.equals()?;
            value = Some(self.identifier_or_number()?);
        }

        Ok((key, value))
    }

    // fn parse_type(&mut self) -> Result<Vec<InterfaceProperty>, String> {
    //     self.identifier()?;
    //     let _type_name = self.identifier()?;
    //     self.equals()?;

    //     todo!();
    // }

    fn parse_interface(&mut self) -> Result<(String, Vec<InterfaceProperty>), String> {
        self.identifier()?;
        let interface_name = self.identifier()?.inner();
        self.brace(Scope::Begin)?;

        let mut properties: Vec<InterfaceProperty> = Vec::new();

        enum TokenFlag {
            Directive,
            Property,
        }

        let mut last_token = TokenFlag::Property;

        loop {
            if let Some(token) = self.lexer.peek() {
                match token {
                    Token::DirectiveStart => {
                        let directives = self.parse_directive()?;

                        match last_token {
                            TokenFlag::Directive => {
                                properties.last_mut().unwrap().directives.extend(directives);
                            }
                            TokenFlag::Property => {
                                let mut property = InterfaceProperty::new(
                                    "".into(),
                                    Token::Identifier("".into()).into(),
                                );
                                property.directives.extend(directives);
                                properties.push(property);
                            }
                        }

                        last_token = TokenFlag::Directive;
                    }
                    Token::Comment(_) => {
                        self.parse_comment()?;
                        continue;
                    }
                    Token::Identifier(_) => {
                        let key = self.identifier()?.inner();
                        self.colon()?;

                        let mut value = self.identifier_or_string()?;
                        let mut values: Vec<Token> = vec![value.clone()];

                        if let Token::Identifier(s) = &value {
                            if let Some(Token::SquareBracket(Scope::Begin)) = self.lexer.peek() {
                                self.square_bracket(Scope::Begin)?;
                                self.square_bracket(Scope::End)?;

                                value = Token::Identifier(format!("{}[]", s));
                            }
                        }

                        match self.lexer.peek() {
                            Some(Token::Dot) => {
                                while let Some(Token::Dot) = self.lexer.peek() {
                                    self.dot()?;
                                    let t = self.identifier()?;
                                    values.push(t);
                                }

                                let last_index = values.len() - 1;
                                if let Token::Identifier(s) = &values[last_index] {
                                    if let Some(Token::SquareBracket(Scope::Begin)) =
                                        self.lexer.peek()
                                    {
                                        self.square_bracket(Scope::Begin)?;
                                        self.square_bracket(Scope::End)?;

                                        values[last_index] = Token::Identifier(format!("{}[]", s));
                                    }
                                }

                                value = Token::Identifier(
                                    values
                                        .into_iter()
                                        .map(|x| x.inner())
                                        .collect::<Vec<String>>()
                                        .join("."),
                                );
                                values = vec![];
                            }
                            Some(Token::Pipe) => {
                                while let Some(Token::Pipe) = self.lexer.peek() {
                                    self.pipe()?;
                                    let t = self.identifier_or_string()?;
                                    values.push(t);
                                }
                            }
                            _ => {}
                        }

                        match last_token {
                            TokenFlag::Directive => {
                                let last_entry = properties.last_mut().unwrap();
                                last_entry.key = key;
                                if values.len() > 1 {
                                    last_entry.value = values.into();
                                } else {
                                    last_entry.value = value.into();
                                }
                            }
                            TokenFlag::Property => {
                                if values.len() > 1 {
                                    properties.push(InterfaceProperty::new(key, values.into()));
                                } else {
                                    properties.push(InterfaceProperty::new(key, value.into()));
                                }
                            }
                        }

                        last_token = TokenFlag::Property;
                    }
                    Token::Brace(Scope::End) => {
                        self.brace(Scope::End)?;
                        break;
                    }
                    _ => {
                        return Err(format!(
                            "Unexpected token: {:?} - {:?}",
                            self.lexer.get_position(),
                            token
                        ))
                    }
                }
            } else {
                return Err("Expected Closing Brace".into());
            }
        }

        Ok((interface_name, properties))
    }

    fn parse_directive(&mut self) -> Result<Vec<Directive>, String> {
        let mut directives: Vec<Directive> = Vec::new();

        loop {
            if let Some(token) = self.lexer.peek() {
                match token {
                    Token::DirectiveStart => {
                        self.lexer.next();
                    }
                    Token::Identifier(_) => {
                        let key = self.identifier()?.inner();
                        self.equals()?;
                        let value = self.string_or_number_or_boolean()?;

                        directives.push(Directive::new(key, value.into()));
                    }
                    Token::DirectiveEnd => {
                        self.lexer.next();
                        break;
                    }
                    _ => {
                        return Err(format!(
                            "Unexpected token: {:?} - {:?}",
                            self.lexer.get_position(),
                            token
                        ))
                    }
                }
            } else {
                return Err("Expected Newline".into());
            }
        }

        Ok(directives)
    }

    fn literal(&mut self) -> Result<String, String> {
        self.quotation_mark()?;
        let literal_name = self.identifier()?.inner();
        self.quotation_mark()?;

        Ok(literal_name)
    }

    fn identifier_or_number(&mut self) -> Result<Token, String> {
        if let Some(token) = self.lexer.peek() {
            match token {
                Token::Identifier(_) => self.identifier(),
                Token::Number(_) => self.number(),
                _ => Err(format!(
                    "Expected Identifier or Number at {:?}",
                    self.lexer.get_position()
                )),
            }
        } else {
            Err(format!(
                "Expected Identifier or Number at {:?}",
                self.lexer.get_position()
            ))
        }
    }

    fn identifier_or_string(&mut self) -> Result<Token, String> {
        if let Some(token) = self.lexer.peek() {
            match token {
                Token::Identifier(_) => self.identifier(),
                Token::QuotationMark(_) => self.string(),
                _ => Err(format!(
                    "Expected Identifier or String at {:?}",
                    self.lexer.get_position()
                )),
            }
        } else {
            Err(format!(
                "Expected Identifier or String at {:?}",
                self.lexer.get_position()
            ))
        }
    }

    fn string_or_number_or_boolean(&mut self) -> Result<Token, String> {
        if let Some(token) = self.lexer.peek() {
            match token {
                Token::Identifier(_) => self.boolean(),
                Token::QuotationMark(_) => self.string(),
                Token::Number(_) => self.number(),
                _ => Err(format!(
                    "Expected String or Number or Boolean at {:?}",
                    self.lexer.get_position()
                )),
            }
        } else {
            Err(format!(
                "String String or Number or Boolean at {:?}",
                self.lexer.get_position()
            ))
        }
    }

    // Base Consumer Functions //

    fn ignore(&mut self) {
        self.lexer.next();
    }

    fn error(&mut self) -> Result<String, String> {
        Err(format!(
            "Unexpected token: {:?} - {:?}",
            self.lexer.get_position(),
            self.lexer.next()
        ))
    }

    fn parse_comment(&mut self) -> Result<String, String> {
        if let Some(Token::Comment(s)) = self.lexer.peek() {
            self.lexer.next();
            Ok(s)
        } else {
            Err(format!(
                "Expected Comment at {:?}",
                self.lexer.get_position()
            ))
        }
    }

    fn identifier(&mut self) -> Result<Token, String> {
        if let Some(Token::Identifier(s)) = self.lexer.peek() {
            self.lexer.next();
            Ok(Token::Identifier(s))
        } else {
            Err(format!(
                "Expected Identifier at {:?}",
                self.lexer.get_position()
            ))
        }
    }

    fn number(&mut self) -> Result<Token, String> {
        if let Some(Token::Number(s)) = self.lexer.peek() {
            self.lexer.next();
            Ok(Token::Number(s))
        } else {
            Err(format!(
                "Expected Number at {:?}",
                self.lexer.get_position()
            ))
        }
    }

    fn boolean(&mut self) -> Result<Token, String> {
        if let Some(Token::Identifier(s)) = self.lexer.peek() {
            if s == "true" || s == "false" {
                self.lexer.next();
                Ok(Token::Boolean(s))
            } else {
                Err(format!(
                    "Expected Boolean at {:?}",
                    self.lexer.get_position()
                ))
            }
        } else {
            Err(format!(
                "Expected Boolean at {:?}",
                self.lexer.get_position()
            ))
        }
    }

    fn string(&mut self) -> Result<Token, String> {
        self.quotation_mark()?;
        if let Some(Token::String(s)) = self.lexer.peek() {
            self.lexer.next();
            self.quotation_mark()?;
            Ok(Token::String(s))
        } else {
            Err(format!(
                "Expected String at {:?}",
                self.lexer.get_position()
            ))
        }
    }

    fn square_bracket(&mut self, scope: Scope) -> Result<(), String> {
        if let Some(Token::SquareBracket(s)) = self.lexer.peek() {
            if s == scope {
                self.lexer.next();
                Ok(())
            } else {
                Err(format!(
                    "Expected {:?} at {:?}",
                    scope,
                    self.lexer.get_position()
                ))
            }
        } else {
            Err(format!(
                "Expected Square Bracket at {:?}",
                self.lexer.get_position()
            ))
        }
    }

    fn brace(&mut self, scope: Scope) -> Result<(), String> {
        if let Some(Token::Brace(s)) = self.lexer.peek() {
            if s == scope {
                self.lexer.next();
                Ok(())
            } else {
                Err(format!(
                    "Expected {:?} at {:?}",
                    scope,
                    self.lexer.get_position()
                ))
            }
        } else {
            Err(format!("Expected Brace at {:?}", self.lexer.get_position()))
        }
    }

    fn quotation_mark(&mut self) -> Result<Token, String> {
        if let Some(Token::QuotationMark(q)) = self.lexer.peek() {
            self.lexer.next();

            match self.lexer.context {
                LexerContext::String => {
                    self.lexer.context = self.lexer.last_context.clone();
                    self.lexer.last_context = LexerContext::String;
                }
                _ => {
                    let temp_context = self.lexer.context.clone();
                    self.lexer.context = LexerContext::String;
                    self.lexer.last_context = temp_context;
                }
            }

            Ok(Token::QuotationMark(q))
        } else {
            Err(format!(
                "Expected Quotation Mark at {:?}",
                self.lexer.get_position()
            ))
        }
    }

    fn dot(&mut self) -> Result<(), String> {
        if let Some(Token::Dot) = self.lexer.peek() {
            self.lexer.next();
            Ok(())
        } else {
            Err(format!("Expected Dot at {:?}", self.lexer.get_position()))
        }
    }

    fn colon(&mut self) -> Result<(), String> {
        if let Some(Token::Colon) = self.lexer.peek() {
            self.lexer.next();
            Ok(())
        } else {
            Err(format!("Expected Colon at {:?}", self.lexer.get_position()))
        }
    }

    fn equals(&mut self) -> Result<(), String> {
        if let Some(Token::Equals) = self.lexer.peek() {
            self.lexer.next();
            Ok(())
        } else {
            Err(format!(
                "Expected Equals at {:?}",
                self.lexer.get_position()
            ))
        }
    }

    fn pipe(&mut self) -> Result<(), String> {
        if let Some(Token::Pipe) = self.lexer.peek() {
            self.lexer.next();
            Ok(())
        } else {
            Err(format!("Expected Pipe at {:?}", self.lexer.get_position()))
        }
    }
}
