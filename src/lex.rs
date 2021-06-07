use std::collections::HashMap;

use crate::token::{Token, TokenType};
pub(crate) struct Lexer<'a> {
    code: &'a [u8],
    current: usize,
    keywords: HashMap<String, TokenType>,
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(code: &'a str) -> Self {
        let mut keywords = HashMap::default();
        keywords.insert("qreg".to_owned(), TokenType::Qreg);
        keywords.insert("creg".to_owned(), TokenType::Creg);
        keywords.insert("include".to_owned(), TokenType::Include);
        keywords.insert("OPENQASM".to_owned(), TokenType::OpenQasm);
        Lexer {
            code: code.as_bytes(),
            current: 0,
            keywords,
        }
    }

    pub(crate) fn lex(&mut self) -> Vec<Token> {
        use crate::token::TokenType::*;
        let mut tokens = vec![];
        while self.current < self.code.len() {
            let start = self.current;
            self.current += 1;
            match self.code[start] as char {
                '[' => tokens.push(Token::new(start, LBracket)),
                ']' => tokens.push(Token::new(start, RBracket)),
                '{' => tokens.push(Token::new(start, LBrace)),
                '}' => tokens.push(Token::new(start, RBrace)),
                '(' => tokens.push(Token::new(start, LParen)),
                ')' => tokens.push(Token::new(start, RParen)),
                ';' => tokens.push(Token::new(start, Semicolon)),

                'a'..='z' | 'A'..='Z' | '_' => tokens.push(self.identifier()),
                '0'..='9' => tokens.push(self.number()),
                _ => {}
            }
        }

        tokens.push(Token::new(0, EOF));
        tokens
    }

    fn number(&mut self) -> Token {
        use crate::token::TokenType::Number;
        let start = self.current - 1;
        while self.peek().is_ascii_digit() {
            self.advance();
        }
        if self.peek() == '.' {
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let num_string = std::str::from_utf8(&self.code[start..self.current]).unwrap();

        Token::new(start, Number(num_string.parse().unwrap()))
    }

    fn identifier(&mut self) -> Token {
        use crate::token::TokenType::Identifier;
        let start = self.current - 1;
        while self.peek().is_ascii_alphanumeric() {
            self.advance();
        }

        let id = std::str::from_utf8(&self.code[start..self.current]).unwrap();

        if let Some(keyword) = self.keywords.get(id) {
            Token::new(start, keyword.clone())
        } else {
            Token::new(start, Identifier(id.to_owned()))
        }
    }

    fn advance(&mut self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.current += 1;
            self.code[self.current - 1] as char
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.code[self.current] as char
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.code.len()
    }
}
