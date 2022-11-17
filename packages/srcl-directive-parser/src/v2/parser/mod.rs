pub mod types;

use crate::{
    v2::lexer::{tokens::Token, Lexer, LexerContext},
    v2::{
        lexer::tokens::Scope,
        parser::types::{Directive, Interface, InterfaceProperty},
    },
};

#[derive(Debug)]
pub struct ParserError {
    pub message: String,
    // pub line: usize,
    // pub column: usize,
}

impl ParserError {
    pub fn new(message: &str) -> ParserError {
        ParserError {
            message: message.to_string(),
            // line: line,
            // column: column,
        }
    }
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Parser<'a> {
        Parser {
            lexer: Lexer::new(input),
        }
    }

    pub fn parse(&mut self) -> Result<Interface, ParserError> {
        while let Some(token) = self.lexer.next() {
            let result = match token {
                Token::Identifier(s) => match s.as_str() {
                    "interface" => self.parse_interface()?,
                    "type" => {
                        todo!();
                    }
                    _ => continue,
                },
                _ => continue,
            };

            return Ok(result);
        }

        Err(ParserError::new("Unexpected EOF"))
    }

    fn parse_interface(&mut self) -> Result<Interface, ParserError> {
        let mut interface = Interface::new();
        interface.name = self.consume(Token::Identifier("".into()))?.inner();

        while let Some(token) = self.lexer.next() {
            println!("{:?}", token);
            match token {
                Token::Identifier(s) => {
                    let property = self.parse_property(s)?;
                    interface.properties.push(property)
                }
                Token::Slash => match self.lexer.prev_nth(1) {
                    Some(Token::Slash) => {
                        if let Some(Token::Star) = self.lexer.peek() {
                            continue;
                        } else {
                            while let Some(t) = self.lexer.next() {
                                match t {
                                    Token::Newline => break,
                                    _ => continue,
                                }
                            }
                        }
                    }
                    _ => {}
                },
                Token::Star => {
                    match (self.lexer.prev_nth(2), self.lexer.prev_nth(1)) {
                        (Some(Token::Slash), Some(Token::Slash)) => {
                            self.parse_directives()?;
                            // self.parse_property()?;
                        }
                        (_, _) => {}
                    }
                }
                Token::Brace(Scope::End) => {
                    return Ok(interface);
                }
                _ => {}
            }
        }

        Ok(interface)
    }

    fn parse_property(&mut self, key: String) -> Result<InterfaceProperty, ParserError> {
        self.consume(Token::Colon)?;
        let value = self.property_value()?;

        let property = InterfaceProperty::new(key, value.into());

        Ok(property)
    }

    fn property_value(&mut self) -> Result<Token, ParserError> {
        let mut token = match self.lexer.peek() {
            Some(Token::QuotationMark(_)) => {
                self.quotation_mark()?;
                let t = self.consume(Token::String("".into()))?;
                self.quotation_mark()?;

                t
            }
            Some(Token::Identifier(_)) => self.consume(Token::Identifier("".into()))?,
            Some(t) => Err(ParserError::new(&format!(
                "Unexpected token while parsing Property Value: {}",
                t.kind()
            )))?,
            _ => Err(ParserError::new("Unexpected EOF"))?,
        };

        token = match &token {
            Token::Identifier(s) => {
                // Handle any kind of brackes after an identifier
                match self.lexer.peek() {
                    Some(Token::SquareBracket(Scope::Begin)) => {
                        self.consume(Token::SquareBracket(Scope::Begin))?;
                        self.consume(Token::SquareBracket(Scope::End))?;

                        Token::Identifier(format!("{}[]", s))
                    }

                    Some(Token::AngleBracket(Scope::Begin)) => {
                        let mut inner = vec![];

                        self.consume(Token::AngleBracket(Scope::Begin))?;
                        inner.push(self.consume(Token::Identifier("".into()))?.inner());

                        while let Some(Token::Comma) = self.lexer.peek() {
                            self.consume(Token::Comma)?;
                            inner.push(self.consume(Token::Identifier("".into()))?.inner());
                        }

                        self.consume(Token::AngleBracket(Scope::End))?;

                        Token::Identifier(format!("{}<{}>", s, inner.join(", ")))
                    }
                    _ => token,
                }
            }
            _ => token,
        };

        let mut temp_tokens: Vec<Token> = vec![token.clone()];

        // Handle Pipe and Dot chains (union types and nested object types)
        match self.lexer.peek() {
            Some(Token::Dot) => {
                while let Some(Token::Dot) = self.lexer.next() {
                    let t = self.consume(Token::Identifier("".into()))?;
                    temp_tokens.push(t);
                }

                token = Token::Identifier(
                    temp_tokens
                        .into_iter()
                        .map(|x| x.inner())
                        .collect::<Vec<String>>()
                        .join("."),
                );
            }
            Some(Token::Pipe) => {
                if let Some(Token::Pipe) = self.lexer.next() {
                    let t = self.property_value()?;
                    temp_tokens.push(t);
                }

                token = Token::Identifier(
                    temp_tokens
                        .into_iter()
                        .map(|x| x.inner())
                        .collect::<Vec<String>>()
                        .join(" | "),
                );
            }
            _ => {}
        }

        Ok(token)
    }

    fn parse_directives(&mut self) -> Result<Vec<Directive>, ParserError> {
        let mut directives: Vec<Directive> = Vec::new();

        while let Some(token) = self.lexer.next() {
            match token {
                Token::Identifier(s) => {
                    let key = s;
                    self.consume(Token::Equals)?;
                    let value = self.directive_value()?;

                    directives.push(Directive::new(key, value.into()));
                }
                Token::Newline => {
                    break;
                }
                _ => {
                    return Err(ParserError::new(&format!(
                        "Unexpected token while parsing Directives: {}",
                        token.kind()
                    )))
                }
            }
        }

        Ok(directives)
    }

    fn directive_value(&mut self) -> Result<Token, ParserError> {
        if let Some(token) = self.lexer.peek() {
            match token {
                Token::QuotationMark(_) => {
                    self.quotation_mark()?;
                    let t = self.consume(Token::String("".into()))?;
                    self.quotation_mark()?;

                    Ok(t)
                }
                Token::Number(_) => self.consume(token),
                Token::Identifier(s) => self.consume(Token::Boolean(s)),
                _ => Err(ParserError::new(&format!(
                    "Unexpected token while parsing Directive Value: {}",
                    token.kind()
                ))),
            }
        } else {
            Err(ParserError::new("Expected Directive Value".into()))
        }
    }

    fn quotation_mark(&mut self) -> Result<Token, ParserError> {
        if let Some(Token::QuotationMark(c)) = self.lexer.next() {
            match self.lexer.context {
                LexerContext::String(_) => {
                    self.lexer.context = LexerContext::Normal;
                    Ok(Token::QuotationMark(c))
                }
                LexerContext::Normal => {
                    self.lexer.context = LexerContext::String(c.clone());
                    Ok(Token::QuotationMark(c))
                }
            }
        } else {
            Err(ParserError::new("Expected Quotation Mark".into()))
        }
    }

    fn consume(&mut self, token: Token) -> Result<Token, ParserError> {
        use std::mem::discriminant;

        if let Some(t) = self.lexer.next() {
            if discriminant(&t) == discriminant(&token) {
                Ok(t)
            } else {
                Err(ParserError::new(&format!(
                    "Expected '{}', found '{}'",
                    token.kind(),
                    t.kind()
                )))
            }
        } else {
            Err(ParserError::new(&format!(
                "Expected '{}', found EOF",
                token.kind()
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() -> Result<(), ParserError> {
        let input = std::fs::read_to_string("tests/fixtures/src.tsx").expect("Could not read file");
        let mut parser = Parser::new(&input);

        let interface = parser.parse()?;
        println!("{:#?}", interface);

        Ok(())
    }
}
