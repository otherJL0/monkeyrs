use crate::token;

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    read_position: usize,
    ch: char,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: char::default(),
        };
        lexer.read_char();
        lexer
    }

    pub fn read_char(&mut self) {
        self.ch = if self.read_position >= self.input.len() {
            char::default()
        } else {
            self.input.chars().nth(self.read_position).unwrap()
        };
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> token::Token {
        let token = match self.ch {
            '=' => token::Token::new(token::TokenType::Assign, "="),
            ';' => token::Token::new(token::TokenType::Semicolon, ";"),
            '(' => token::Token::new(token::TokenType::LeftParen, "("),
            ')' => token::Token::new(token::TokenType::RightParen, ")"),
            ',' => token::Token::new(token::TokenType::Comma, ","),
            '+' => token::Token::new(token::TokenType::Plus, "+"),
            '{' => token::Token::new(token::TokenType::LeftBrace, "{"),
            '}' => token::Token::new(token::TokenType::RightBrace, "}"),
            _ => token::Token::new(token::TokenType::Eof, ""),
        };
        self.read_char();
        token
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct TestCase<'a> {
        expected_type: token::TokenType,
        expected_literal: &'a str,
    }
    impl<'a> TestCase<'a> {
        pub fn new(expected_type: token::TokenType, expected_literal: &'a str) -> TestCase<'a> {
            TestCase {
                expected_type,
                expected_literal,
            }
        }
    }

    #[test]
    fn test_next_token() {
        let test_cases = [
            TestCase::new(token::TokenType::Assign, "="),
            TestCase::new(token::TokenType::Plus, "+"),
            TestCase::new(token::TokenType::LeftParen, "("),
            TestCase::new(token::TokenType::RightParen, ")"),
            TestCase::new(token::TokenType::LeftBrace, "{"),
            TestCase::new(token::TokenType::RightBrace, "}"),
            TestCase::new(token::TokenType::Comma, ","),
            TestCase::new(token::TokenType::Semicolon, ";"),
            TestCase::new(token::TokenType::Eof, ""),
        ];

        let input = "=+(){},;";
        let mut lexer = Lexer::new(input);
        for (i, tt) in test_cases.into_iter().enumerate() {
            let token = lexer.next_token();
            assert_eq!(token.token_type, tt.expected_type);
            assert_eq!(token.literal, tt.expected_literal);
        }
    }
}
