#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum TokenType {
    Illegal,
    Eof,

    // Identifiers and Literals
    Ident,
    Int,

    // Operators
    Assign,
    Plus,

    // Delimiters
    Comma,
    Semicolon,

    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,

    // Keywords
    Function,
    Let,
}

pub struct Token<'a> {
    pub token_type: TokenType,
    pub literal: &'a str,
}

impl<'a> Token<'a> {
    pub fn new(token_type: TokenType, literal: &'a str) -> Token<'a> {
        Token {
            token_type,
            literal,
        }
    }
}
