#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
    Bang,
    Asterisk,
    Slash,
    PlusEqual,
    MinusEqual,
    AsteriskEqual,
    SlashEqual,

    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    BangEqual,
    EqualEqual,

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
    True,
    False,
    If,
    Else,
    Return,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn is_type(&self, target_type: TokenType) -> bool {
        self.token_type == target_type
    }
}

impl Token {
    pub fn new(token_type: TokenType, literal: &str) -> Token {
        Token {
            token_type,
            literal: literal.to_string(),
        }
    }
}
