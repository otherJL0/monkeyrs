#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum TokenType {
    Illegal,
    Eof,

    // Identifiers and Literals
    Identifier,
    Int,

    // Operators
    Assign,
    Plus,
    Minus,
    PlusEqual,
    MinusEqual,

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

    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    EqualEqual,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Token {
        Token {
            token_type,
            literal,
        }
    }
}
