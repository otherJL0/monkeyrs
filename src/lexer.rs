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
        lexer.advance();
        lexer
    }

    fn peek(&self) -> char {
        self.input.chars().nth(self.read_position).unwrap()
    }

    pub fn advance(&mut self) {
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
            self.advance();
        }
        let literal = &self.input[start..self.position];
        Token::new(TokenType::Int, literal.to_string())
    }
    fn read_identifier(&mut self) -> Token {
        let start = self.position;
        while self.ch.is_ascii_alphabetic() {
            self.advance();
        }
        let literal = &self.input[start..self.position];
        let token_type = match literal {
            "fn" => TokenType::Function,
            "let" => TokenType::Let,
            "true" => TokenType::True,
            "false" => TokenType::False,
            _ => TokenType::Identifier,
        };
        Token::new(token_type, String::from(literal))
    }

    pub fn next_token(&mut self) -> Token {
        while self.ch.is_whitespace() {
            self.advance();
        }
        let token = match self.ch {
            '=' => {
                if self.peek() == '=' {
                    self.advance();
                    Token::new(TokenType::EqualEqual, String::from("=="))
                } else {
                    Token::new(TokenType::Assign, String::from("="))
                }
            }
            '<' => {
                if self.peek() == '=' {
                    self.advance();
                    Token::new(TokenType::LessEqual, String::from("<="))
                } else {
                    Token::new(TokenType::Less, String::from("<"))
                }
            }
            '>' => {
                if self.peek() == '=' {
                    self.advance();
                    Token::new(TokenType::GreaterEqual, String::from(">="))
                } else {
                    Token::new(TokenType::Greater, String::from(">"))
                }
            }
            '+' => {
                if self.peek() == '=' {
                    self.advance();
                    Token::new(TokenType::PlusEqual, String::from("+="))
                } else {
                    Token::new(TokenType::Plus, String::from("+"))
                }
            }
            '-' => {
                if self.peek() == '=' {
                    self.advance();
                    Token::new(TokenType::MinusEqual, String::from("-="))
                } else {
                    Token::new(TokenType::Minus, String::from("-"))
                }
            }
            ';' => Token::new(TokenType::Semicolon, String::from(";")),
            '(' => Token::new(TokenType::LeftParen, String::from("(")),
            ')' => Token::new(TokenType::RightParen, String::from(")")),
            ',' => Token::new(TokenType::Comma, String::from(",")),
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
        self.advance();
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
    fn test_simple_function() {
        let expected_tokens = [
            Token::new(TokenType::Let, String::from("let")),
            Token::new(TokenType::Identifier, String::from("five")),
            Token::new(TokenType::Assign, String::from("=")),
            Token::new(TokenType::Int, String::from("5")),
            Token::new(TokenType::Semicolon, String::from(";")),
            Token::new(TokenType::Let, String::from("let")),
            Token::new(TokenType::Identifier, String::from("ten")),
            Token::new(TokenType::Assign, String::from("=")),
            Token::new(TokenType::Int, String::from("10")),
            Token::new(TokenType::Semicolon, String::from(";")),
            Token::new(TokenType::Let, String::from("let")),
            Token::new(TokenType::Identifier, String::from("add")),
            Token::new(TokenType::Assign, String::from("=")),
            Token::new(TokenType::Function, String::from("fn")),
            Token::new(TokenType::LeftParen, String::from("(")),
            Token::new(TokenType::Identifier, String::from("x")),
            Token::new(TokenType::Comma, String::from(",")),
            Token::new(TokenType::Identifier, String::from("y")),
            Token::new(TokenType::RightParen, String::from(")")),
            Token::new(TokenType::LeftBrace, String::from("{")),
            Token::new(TokenType::Identifier, String::from("x")),
            Token::new(TokenType::Plus, String::from("+")),
            Token::new(TokenType::Identifier, String::from("y")),
            Token::new(TokenType::Semicolon, String::from(";")),
            Token::new(TokenType::RightBrace, String::from("}")),
            Token::new(TokenType::Semicolon, String::from(";")),
            Token::new(TokenType::Let, String::from("let")),
            Token::new(TokenType::Identifier, String::from("result")),
            Token::new(TokenType::Assign, String::from("=")),
            Token::new(TokenType::Identifier, String::from("add")),
            Token::new(TokenType::LeftParen, String::from("(")),
            Token::new(TokenType::Identifier, String::from("five")),
            Token::new(TokenType::Comma, String::from(",")),
            Token::new(TokenType::Identifier, String::from("ten")),
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

    #[test]
    fn test_increment_decrement() {
        let input = r"let a = 10;
        a += 8;
        a -= 5;
        ";
        let expected_tokens = [
            Token::new(TokenType::Let, String::from("let")),
            Token::new(TokenType::Identifier, String::from("a")),
            Token::new(TokenType::Assign, String::from("=")),
            Token::new(TokenType::Int, String::from("10")),
            Token::new(TokenType::Semicolon, String::from(";")),
            Token::new(TokenType::Identifier, String::from("a")),
            Token::new(TokenType::PlusEqual, String::from("+=")),
            Token::new(TokenType::Int, String::from("8")),
            Token::new(TokenType::Semicolon, String::from(";")),
            Token::new(TokenType::Identifier, String::from("a")),
            Token::new(TokenType::MinusEqual, String::from("-=")),
            Token::new(TokenType::Int, String::from("5")),
            Token::new(TokenType::Semicolon, String::from(";")),
        ];
        let mut lexer = Lexer::new(input);
        for expected_token in expected_tokens {
            assert_eq!(lexer.next_token(), expected_token);
            println!("Passed {expected_token:?}");
        }
    }
    #[test]
    fn test_comparison() {
        let input = r"let a = 10;
        a >= 7 == true;
        a <= 4 == false;
        ";
        let expected_tokens = [
            Token::new(TokenType::Let, String::from("let")),
            Token::new(TokenType::Identifier, String::from("a")),
            Token::new(TokenType::Assign, String::from("=")),
            Token::new(TokenType::Int, String::from("10")),
            Token::new(TokenType::Semicolon, String::from(";")),
            Token::new(TokenType::Identifier, String::from("a")),
            Token::new(TokenType::GreaterEqual, String::from(">=")),
            Token::new(TokenType::Int, String::from("7")),
            Token::new(TokenType::EqualEqual, String::from("==")),
            Token::new(TokenType::True, String::from("true")),
            Token::new(TokenType::Semicolon, String::from(";")),
            Token::new(TokenType::Identifier, String::from("a")),
            Token::new(TokenType::LessEqual, String::from("<=")),
            Token::new(TokenType::Int, String::from("4")),
            Token::new(TokenType::EqualEqual, String::from("==")),
            Token::new(TokenType::False, String::from("false")),
            Token::new(TokenType::Semicolon, String::from(";")),
        ];
        let mut lexer = Lexer::new(input);
        for expected_token in expected_tokens {
            assert_eq!(lexer.next_token(), expected_token);
            println!("Passed {expected_token:?}");
        }
    }
}
