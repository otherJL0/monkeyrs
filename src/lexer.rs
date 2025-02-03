use crate::token::{Token, TokenType};

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

    fn read_number(&mut self) -> Token {
        let start = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        let literal = &self.input[start..self.position];
        Token::new(TokenType::Int, literal.to_string())
    }
    fn read_identifier(&mut self) -> Token {
        let start = self.position;
        while self.ch.is_ascii_alphabetic() {
            self.read_char();
        }
        let literal = &self.input[start..self.position];
        let token_type = match literal {
            "fn" => TokenType::Function,
            "let" => TokenType::Let,
            _ => TokenType::Ident,
        };
        Token::new(token_type, String::from(literal))
    }

    pub fn next_token(&mut self) -> Token {
        while self.ch.is_whitespace() {
            self.read_char();
        }
        let token = match self.ch {
            '=' => Token::new(TokenType::Assign, String::from("=")),
            ';' => Token::new(TokenType::Semicolon, String::from(";")),
            '(' => Token::new(TokenType::LeftParen, String::from("(")),
            ')' => Token::new(TokenType::RightParen, String::from(")")),
            ',' => Token::new(TokenType::Comma, String::from(",")),
            '+' => Token::new(TokenType::Plus, String::from("+")),
            '{' => Token::new(TokenType::LeftBrace, String::from("{")),
            '}' => Token::new(TokenType::RightBrace, String::from("}")),
            '\0' => Token::new(TokenType::Eof, String::default()),
            _ => {
                if self.ch.is_ascii_alphabetic() {
                    return self.read_identifier();
                }
                if self.ch.is_ascii_digit() {
                    return self.read_number();
                }
                return Token::new(TokenType::Illegal, String::from(self.ch));
            }
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
            Token::new(TokenType::Assign, String::from("=")),
            Token::new(TokenType::Plus, String::from("+")),
            Token::new(TokenType::LeftParen, String::from("(")),
            Token::new(TokenType::RightParen, String::from(")")),
            Token::new(TokenType::LeftBrace, String::from("{")),
            Token::new(TokenType::RightBrace, String::from("}")),
            Token::new(TokenType::Comma, String::from(",")),
            Token::new(TokenType::Semicolon, String::from(";")),
            Token::new(TokenType::Eof, String::from("")),
        ];

        let input = "=+(){},;";
        let mut lexer = Lexer::new(input);
        for expected_token in expected_tokens {
            assert_eq!(lexer.next_token(), expected_token);
        }
    }

    #[test]
    fn test_simple_expression() {
        let expected_tokens = [
            Token::new(TokenType::Let, String::from("let")),
            Token::new(TokenType::Ident, String::from("five")),
            Token::new(TokenType::Assign, String::from("=")),
            Token::new(TokenType::Int, String::from("5")),
            Token::new(TokenType::Semicolon, String::from(";")),
            Token::new(TokenType::Let, String::from("let")),
            Token::new(TokenType::Ident, String::from("ten")),
            Token::new(TokenType::Assign, String::from("=")),
            Token::new(TokenType::Int, String::from("10")),
            Token::new(TokenType::Semicolon, String::from(";")),
            Token::new(TokenType::Let, String::from("let")),
            Token::new(TokenType::Ident, String::from("add")),
            Token::new(TokenType::Assign, String::from("=")),
            Token::new(TokenType::Function, String::from("fn")),
            Token::new(TokenType::LeftParen, String::from("(")),
            Token::new(TokenType::Ident, String::from("x")),
            Token::new(TokenType::Comma, String::from(",")),
            Token::new(TokenType::Ident, String::from("y")),
            Token::new(TokenType::RightParen, String::from(")")),
            Token::new(TokenType::LeftBrace, String::from("{")),
            Token::new(TokenType::Ident, String::from("x")),
            Token::new(TokenType::Plus, String::from("+")),
            Token::new(TokenType::Ident, String::from("y")),
            Token::new(TokenType::Semicolon, String::from(";")),
            Token::new(TokenType::RightBrace, String::from("}")),
            Token::new(TokenType::Semicolon, String::from(";")),
            Token::new(TokenType::Let, String::from("let")),
            Token::new(TokenType::Ident, String::from("result")),
            Token::new(TokenType::Assign, String::from("=")),
            Token::new(TokenType::Ident, String::from("add")),
            Token::new(TokenType::LeftParen, String::from("(")),
            Token::new(TokenType::Ident, String::from("five")),
            Token::new(TokenType::Comma, String::from(",")),
            Token::new(TokenType::Ident, String::from("ten")),
            Token::new(TokenType::RightParen, String::from(")")),
            Token::new(TokenType::Semicolon, String::from(";")),
            Token::new(TokenType::Eof, String::from("")),
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
            assert_eq!(lexer.next_token(), expected_token);
            println!("Passed {expected_token:?}");
        }
    }
}
