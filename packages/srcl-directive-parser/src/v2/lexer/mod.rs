pub mod tokens;

use crate::v2::lexer::tokens::{Scope, Token};
use itertools::Itertools;
use std::{iter::Peekable, str::Chars};

#[derive(Debug, Clone)]
pub enum LexerContext {
    Normal,
    String(char),
}

pub struct Lexer<'a> {
    input: &'a str,
    input_iter: Peekable<Chars<'a>>,
    tokens: Vec<Token>,
    token_index: isize,
    pub context: LexerContext,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer {
            input,
            input_iter: input.chars().peekable(),
            tokens: Vec::new(),
            token_index: -1,
            context: LexerContext::Normal,
        }
    }

    pub fn peek(&mut self) -> Option<Token> {
        if self.token_index + 1 == self.tokens.len() as isize {
            let token = self.next();
            self.token_index -= 1;
            token
        } else {
            self.tokens
                .get((self.token_index + 1) as usize)
                .map(|x| x.clone())
        }
    }

    pub fn next(&mut self) -> Option<Token> {
        if self.token_index + 1 < self.tokens.len() as isize {
            let token = self
                .tokens
                .get((self.token_index + 1) as usize)
                .map(|x| x.clone());

            self.token_index += 1;

            return token;
        }

        if let Some(&c) = self.input_iter.peek() {
            let current_token = match self.context {
                LexerContext::Normal => self.lex_char(c),
                LexerContext::String(q) => self.lex_string_char(c, q),
            };

            if matches!(current_token, Token::Whitespace) {
                self.next()
            } else {
                self.tokens
                    .get(self.token_index as usize)
                    .map(|x| x.clone())
            }
        } else {
            None
        }
    }

    pub fn prev_nth(&self, n: usize) -> Option<Token> {
        if self.token_index >= n as isize {
            self.tokens
                .get(self.token_index as usize - n)
                .map(|x| x.clone())
        } else {
            None
        }
    }

    fn lex_char(&mut self, c: char) -> Token {
        match c {
            c if c.is_alphabetic() => {
                let identifier = self.take_and_fold_while(|&c| c.is_alphanumeric() || c == '_');
                let token = Token::Identifier(identifier);

                self.tokens.push(token.clone());
                self.token_index += 1;

                token
            }
            c if c.is_digit(10) => {
                let number = self.take_and_fold_while(|&c| c.is_digit(10) || c == '.');
                let token = Token::Number(number);

                self.tokens.push(token.clone());
                self.token_index += 1;

                token
            }
            '/' => self.consume(Token::Slash),
            '*' => self.consume(Token::Star),
            '=' => self.consume(Token::Equals),
            ' ' => self.consume(Token::Whitespace),
            '"' => self.consume(Token::QuotationMark('"')),
            '\'' => self.consume(Token::QuotationMark('\'')),
            '\n' => self.consume(Token::Newline),
            '\\' => self.consume(Token::Backslash),
            '-' => self.consume(Token::Dash),
            '_' => self.consume(Token::Underscore),
            '{' => self.consume(Token::Brace(Scope::Begin)),
            '}' => self.consume(Token::Brace(Scope::End)),
            '[' => self.consume(Token::SquareBracket(Scope::Begin)),
            ']' => self.consume(Token::SquareBracket(Scope::End)),
            '<' => self.consume(Token::AngleBracket(Scope::Begin)),
            '>' => self.consume(Token::AngleBracket(Scope::End)),
            '(' => self.consume(Token::Parentheses(Scope::Begin)),
            ')' => self.consume(Token::Parentheses(Scope::End)),
            '|' => self.consume(Token::Pipe),
            '.' => self.consume(Token::Dot),
            ',' => self.consume(Token::Comma),
            ':' => self.consume(Token::Colon),
            ';' => self.consume(Token::Semicolon),
            '?' => self.consume(Token::QuestionMark),
            '&' => self.consume(Token::Ampersand),
            _ => panic!("Invalid Character found: \"{}\"", c),
        }
    }

    fn lex_string_char(&mut self, c: char, q: char) -> Token {
        match c {
            '"' => self.consume(Token::QuotationMark('"')),
            '\'' => self.consume(Token::QuotationMark('\'')),
            '\n' => self.consume(Token::Newline),
            _ => {
                // TODO: Check for escaped quotation marks in strings
                let string = self.take_and_fold_while(|&c| c != q && c != '\n');
                let token = Token::String(string);

                self.tokens.push(token.clone());
                self.token_index += 1;

                token
            }
        }
    }

    fn consume(&mut self, token: Token) -> Token {
        self.input_iter.next();

        if !matches!(token, Token::Whitespace) {
            self.tokens.push(token.clone());
            self.token_index += 1;
        }

        token
    }

    fn take_and_fold_while<F>(&mut self, predicate: F) -> String
    where
        F: FnMut(&char) -> bool,
    {
        self.input_iter
            .by_ref()
            .peeking_take_while(predicate)
            .fold(String::new(), |acc, c| acc + String::from(c).as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_v2() {
        let input = std::fs::read_to_string("tests/fixtures/src.tsx").expect("Could not read file");
        let mut lexer = Lexer::new(&input);
        while let Some(_t) = lexer.next() {}
    }
}
