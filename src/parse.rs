use crate::token::{
    Token,
    TokenType::{self, *},
};

pub(crate) struct Parser {
    pub(crate) tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub(crate) fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub(crate) fn parse(mut self) {
        while !self.is_at_end() {
            self.statement();
        }
    }

    pub(crate) fn statement(&mut self) {
        if (self.mat(Qreg)) {}
    }

    fn is_at_end(&self) -> bool {
        self.tokens[self.current].token_type == TokenType::EOF
    }

    fn mat(&mut self, token_type: TokenType) -> bool {
        if !self.is_at_end() && self.tokens[self.current].token_type == token_type {
            self.current += 1;
            true
        } else {
            false
        }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }
}

pub(crate) struct AST {}
