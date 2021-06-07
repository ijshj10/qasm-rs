#[derive(Debug)]
pub(crate) struct Token {
    pub(crate) offset: usize,
    pub(crate) token_type: TokenType,
}

impl Token {
    pub(crate) fn new(offset: usize, token_type: TokenType) -> Self {
        Token { offset, token_type }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Single chracter token
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Semicolon,

    // keywords
    Qreg,
    Creg,
    Include,
    OpenQasm,

    Number(f64),
    Identifier(String),

    EOF,
}
