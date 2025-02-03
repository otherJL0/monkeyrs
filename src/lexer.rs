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

    #[test]
    fn test_punctuation() {
        let expected_tokens = [
            token::Token::new(token::TokenType::Assign, "="),
            token::Token::new(token::TokenType::Plus, "+"),
            token::Token::new(token::TokenType::LeftParen, "("),
            token::Token::new(token::TokenType::RightParen, ")"),
            token::Token::new(token::TokenType::LeftBrace, "{"),
            token::Token::new(token::TokenType::RightBrace, "}"),
            token::Token::new(token::TokenType::Comma, ","),
            token::Token::new(token::TokenType::Semicolon, ";"),
            token::Token::new(token::TokenType::Eof, ""),
        ];

        let input = "=+(){},;";
        let mut lexer = Lexer::new(input);
        for expected_token in expected_tokens {
            let token = lexer.next_token();
            assert_eq!(token.token_type, expected_token.token_type);
            assert_eq!(token.literal, expected_token.literal);
        }
    }

    #[test]
    fn test_simple_expression() {
        let expected_tokens = [
            token::Token::new(token::TokenType::Let, "let"),
            token::Token::new(token::TokenType::Ident, "five"),
            token::Token::new(token::TokenType::Assign, "="),
            token::Token::new(token::TokenType::Int, "5"),
            token::Token::new(token::TokenType::Semicolon, ";"),
            token::Token::new(token::TokenType::Let, "let"),
            token::Token::new(token::TokenType::Ident, "ten"),
            token::Token::new(token::TokenType::Assign, "="),
            token::Token::new(token::TokenType::Int, "10"),
            token::Token::new(token::TokenType::Semicolon, ";"),
            token::Token::new(token::TokenType::Let, "let"),
            token::Token::new(token::TokenType::Ident, "add"),
            token::Token::new(token::TokenType::Assign, "="),
            token::Token::new(token::TokenType::Function, "fn"),
            token::Token::new(token::TokenType::LeftParen, "("),
            token::Token::new(token::TokenType::Ident, "x"),
            token::Token::new(token::TokenType::Comma, ","),
            token::Token::new(token::TokenType::Ident, "y"),
            token::Token::new(token::TokenType::RightParen, ")"),
            token::Token::new(token::TokenType::LeftBrace, "{"),
            token::Token::new(token::TokenType::Ident, "x"),
            token::Token::new(token::TokenType::Plus, "+"),
            token::Token::new(token::TokenType::Ident, "y"),
            token::Token::new(token::TokenType::Semicolon, ";"),
            token::Token::new(token::TokenType::RightBrace, "}"),
            token::Token::new(token::TokenType::Semicolon, ";"),
            token::Token::new(token::TokenType::Let, "let"),
            token::Token::new(token::TokenType::Ident, "result"),
            token::Token::new(token::TokenType::Assign, "="),
            token::Token::new(token::TokenType::Ident, "add"),
            token::Token::new(token::TokenType::LeftParen, "("),
            token::Token::new(token::TokenType::Ident, "five"),
            token::Token::new(token::TokenType::Comma, ","),
            token::Token::new(token::TokenType::Ident, "ten"),
            token::Token::new(token::TokenType::RightParen, ")"),
            token::Token::new(token::TokenType::Semicolon, ";"),
            token::Token::new(token::TokenType::Eof, ""),
        ];
        let input = r"let five = 5;
        let ten = 10;

        let add = fn(x, y) {
          x + y;
        };
        let result = add(five, ten);
        ";
        let mut lexer = Lexer::new(input);
        for expected_token in expected_tokens {
            let token = lexer.next_token();
            assert_eq!(token.token_type, expected_token.token_type);
            assert_eq!(token.literal, expected_token.literal);
        }
    }
}
