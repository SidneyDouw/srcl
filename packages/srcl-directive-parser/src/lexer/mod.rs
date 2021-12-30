pub mod types;

use crate::lexer::types::{Scope, Token};

#[derive(Debug, Clone)]
pub enum LexerContext {
    Normal,
    Directive,
    String,
}

#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    pub context: LexerContext,
    pub last_context: LexerContext,
    col_range: (usize, usize),
    line_number: usize,
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.col_range.0 = self.col_range.1;

        if let Some(c) = self.peek_char() {
            let token = match self.context {
                LexerContext::String => match c {
                    '"' => self.quotation_mark(),
                    '\'' => self.quotation_mark(),
                    '\n' => self.newline(),
                    _ => self.string(),
                },

                _ => match c {
                    c if c.is_alphabetic() => self.identifier(),
                    c if c.is_digit(10) => self.number(),
                    '/' => self.slash(),
                    '=' => self.equals(),
                    ' ' => self.whitespace(),
                    '"' => self.quotation_mark(),
                    '\'' => self.quotation_mark(),
                    '\n' => self.newline(),
                    '\\' => self.consume(Token::Backslash),
                    '-' => self.consume(Token::Dash),
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
                    _ => panic!("Invalid Character found: \"{}\"", c),
                },
            };

            if matches!(
                token,
                Token::Whitespace | Token::Newline | Token::Semicolon | Token::Comma
            ) {
                self.next()
            } else {
                Some(token)
            }
        } else {
            None
        }
    }
}

impl Lexer<'_> {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            input,
            position: 0,
            context: LexerContext::Normal,
            last_context: LexerContext::Normal,
            line_number: 1,
            col_range: (1, 1),
        }
    }

    pub fn get_position(&self) -> (usize, (usize, usize)) {
        (self.line_number, self.col_range)
    }

    pub fn peek(&mut self) -> Option<Token> {
        self.peek_nth(0)
    }

    fn peek_nth(&mut self, i: u8) -> Option<Token> {
        let prev_peek_position = self.position;
        let prev_line_number = self.line_number;
        let prev_col_range = self.col_range;
        let prev_context = self.context.clone();

        let mut token: Option<Token> = None;
        for _ in 0..=i {
            token = self.next();
        }

        self.position = prev_peek_position;
        self.line_number = prev_line_number;
        self.col_range = prev_col_range;
        self.context = prev_context;

        token
    }

    fn peek_char(&self) -> Option<char> {
        let ch = self.input.chars().nth(self.position);
        ch
    }

    fn peek_nth_char(&mut self, i: u8) -> Option<char> {
        let prev_peek_position = self.position;
        let prev_line_number = self.line_number;
        let prev_col_range = self.col_range;
        let prev_context = self.context.clone();

        let mut c: Option<char> = None;
        for _ in 0..=i {
            c = self.peek_char_and_advance()
        }

        self.position = prev_peek_position;
        self.line_number = prev_line_number;
        self.col_range = prev_col_range;
        self.context = prev_context;

        c
    }

    fn peek_char_and_advance(&mut self) -> Option<char> {
        let ch = self.peek_char();

        self.position += 1;
        self.col_range.1 += 1;

        ch
    }

    // Consumers

    fn consume(&mut self, t: Token) -> Token {
        self.peek_char_and_advance();
        t
    }

    fn identifier(&mut self) -> Token {
        let mut identifier = String::from(self.peek_char_and_advance().unwrap());

        while let Some(c) = self.peek_char() {
            if c.is_alphanumeric() || c == '_' {
                identifier.push(self.peek_char_and_advance().unwrap());
            } else {
                break;
            }
        }

        Token::Identifier(identifier)
    }

    fn number(&mut self) -> Token {
        let mut number = String::new();

        while let Some(c) = self.peek_char() {
            if c.is_digit(10) || c == '.' {
                number.push(self.peek_char_and_advance().unwrap());
            } else {
                break;
            }
        }

        Token::Number(number)
    }

    fn string(&mut self) -> Token {
        let mut string = String::new();

        while let Some(c) = self.peek_char() {
            if c != '"' && c != '\'' {
                string.push(self.peek_char_and_advance().unwrap());
            } else {
                break;
            }
        }

        Token::String(string)
    }

    fn slash(&mut self) -> Token {
        // let c0 = '/';
        let c1 = self.peek_nth_char(1).expect("Unexpected End");
        let c2 = self.peek_nth_char(2).unwrap_or(' ');

        match (c1, c2) {
            ('/', '*') => {
                self.peek_char_and_advance();
                self.peek_char_and_advance();
                self.peek_char_and_advance();

                self.context = LexerContext::Directive;
                Token::DirectiveStart
            }
            ('/', _) => self.comment(),
            (_, _) => {
                self.peek_char_and_advance();
                Token::Slash
                // panic!("Invalid character after first slash: {}", c)
            }
        }
    }

    fn quotation_mark(&mut self) -> Token {
        let c = self.peek_char_and_advance().unwrap();

        Token::QuotationMark(c)
    }

    fn comment(&mut self) -> Token {
        self.peek_char_and_advance();
        self.peek_char_and_advance();

        let mut comment = String::new();

        while let Some(c) = self.peek_char() {
            if c == '\n' {
                break;
            }

            comment.push(self.peek_char_and_advance().unwrap());
        }

        Token::Comment(comment.trim().to_string())
    }

    fn equals(&mut self) -> Token {
        self.peek_char_and_advance();

        if let Some(c) = self.peek_char() {
            if c == '>' {
                self.consume(Token::FatArrow)
            } else {
                Token::Equals
            }
        } else {
            panic!("Unexpected End")
        }
    }

    fn whitespace(&mut self) -> Token {
        while let Some(c) = self.peek_char() {
            if c == ' ' {
                self.peek_char_and_advance();
            } else {
                break;
            }
        }

        Token::Whitespace
    }

    fn newline(&mut self) -> Token {
        match self.context {
            LexerContext::Directive => {
                self.context = LexerContext::Normal;
                Token::DirectiveEnd
            }
            _ => {
                self.peek_char_and_advance();

                self.col_range = (1, 1);
                self.line_number += 1;

                Token::Newline
            }
        }
    }
}
