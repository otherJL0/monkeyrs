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
        Token::new(TokenType::Int, literal)
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
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "return" => TokenType::Return,
            _ => TokenType::Identifier,
        };
        Token::new(token_type, literal)
    }

    pub fn next_token(&mut self) -> Token {
        while self.ch.is_whitespace() {
            self.advance();
        }
        let token = match self.ch {
            '=' => {
                if self.peek() == '=' {
                    self.advance();
                    Token::new(TokenType::EqualEqual, "==")
                } else {
                    Token::new(TokenType::Assign, "=")
                }
            }
            '<' => {
                if self.peek() == '=' {
                    self.advance();
                    Token::new(TokenType::LessEqual, "<=")
                } else {
                    Token::new(TokenType::Less, "<")
                }
            }
            '>' => {
                if self.peek() == '=' {
                    self.advance();
                    Token::new(TokenType::GreaterEqual, ">=")
                } else {
                    Token::new(TokenType::Greater, ">")
                }
            }
            '+' => {
                if self.peek() == '=' {
                    self.advance();
                    Token::new(TokenType::PlusEqual, "+=")
                } else {
                    Token::new(TokenType::Plus, "+")
                }
            }
            '-' => {
                if self.peek() == '=' {
                    self.advance();
                    Token::new(TokenType::MinusEqual, "-=")
                } else {
                    Token::new(TokenType::Minus, "-")
                }
            }
            '*' => {
                if self.peek() == '=' {
                    self.advance();
                    Token::new(TokenType::AsteriskEqual, "*=")
                } else {
                    Token::new(TokenType::Asterisk, "(")
                }
            }
            '/' => {
                if self.peek() == '=' {
                    self.advance();
                    Token::new(TokenType::SlashEqual, "/=")
                } else {
                    Token::new(TokenType::Slash, "/")
                }
            }
            '!' => {
                if self.peek() == '=' {
                    self.advance();
                    Token::new(TokenType::BangEqual, "!=")
                } else {
                    Token::new(TokenType::Bang, "!")
                }
            }
            ';' => Token::new(TokenType::Semicolon, ";"),
            '(' => Token::new(TokenType::LeftParen, "("),
            ')' => Token::new(TokenType::RightParen, ")"),
            ',' => Token::new(TokenType::Comma, ","),
            '{' => Token::new(TokenType::LeftBrace, "{"),
            '}' => Token::new(TokenType::RightBrace, "}"),
            '\0' => Token::new(TokenType::Eof, ""),
            _ => {
                if self.ch.is_ascii_alphabetic() {
                    return self.read_identifier();
                }
                if self.ch.is_ascii_digit() {
                    return self.read_number();
                }
                return Token::new(
                    TokenType::Illegal,
                    std::str::from_utf8(&[self.ch as u8]).unwrap(),
                );
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
            Token::new(TokenType::Assign, "="),
            Token::new(TokenType::Plus, "+"),
            Token::new(TokenType::LeftParen, "("),
            Token::new(TokenType::RightParen, ")"),
            Token::new(TokenType::LeftBrace, "{"),
            Token::new(TokenType::RightBrace, "}"),
            Token::new(TokenType::Comma, ","),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::Eof, ""),
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
            Token::new(TokenType::Let, "let"),
            Token::new(TokenType::Identifier, "five"),
            Token::new(TokenType::Assign, "="),
            Token::new(TokenType::Int, "5"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::Let, "let"),
            Token::new(TokenType::Identifier, "ten"),
            Token::new(TokenType::Assign, "="),
            Token::new(TokenType::Int, "10"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::Let, "let"),
            Token::new(TokenType::Identifier, "add"),
            Token::new(TokenType::Assign, "="),
            Token::new(TokenType::Function, "fn"),
            Token::new(TokenType::LeftParen, "("),
            Token::new(TokenType::Identifier, "x"),
            Token::new(TokenType::Comma, ","),
            Token::new(TokenType::Identifier, "y"),
            Token::new(TokenType::RightParen, ")"),
            Token::new(TokenType::LeftBrace, "{"),
            Token::new(TokenType::Identifier, "x"),
            Token::new(TokenType::Plus, "+"),
            Token::new(TokenType::Identifier, "y"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::RightBrace, "}"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::Let, "let"),
            Token::new(TokenType::Identifier, "result"),
            Token::new(TokenType::Assign, "="),
            Token::new(TokenType::Identifier, "add"),
            Token::new(TokenType::LeftParen, "("),
            Token::new(TokenType::Identifier, "five"),
            Token::new(TokenType::Comma, ","),
            Token::new(TokenType::Identifier, "ten"),
            Token::new(TokenType::RightParen, ")"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::Eof, ""),
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
            Token::new(TokenType::Let, "let"),
            Token::new(TokenType::Identifier, "a"),
            Token::new(TokenType::Assign, "="),
            Token::new(TokenType::Int, "10"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::Identifier, "a"),
            Token::new(TokenType::PlusEqual, "+="),
            Token::new(TokenType::Int, "8"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::Identifier, "a"),
            Token::new(TokenType::MinusEqual, "-="),
            Token::new(TokenType::Int, "5"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::Eof, ""),
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
        a != 5;
        a /= 2;
        a == 5;
        a *= 1;
        !(a == 5) == false;
        ";
        let expected_tokens = [
            Token::new(TokenType::Let, "let"),
            Token::new(TokenType::Identifier, "a"),
            Token::new(TokenType::Assign, "="),
            Token::new(TokenType::Int, "10"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::Identifier, "a"),
            Token::new(TokenType::GreaterEqual, ">="),
            Token::new(TokenType::Int, "7"),
            Token::new(TokenType::EqualEqual, "=="),
            Token::new(TokenType::True, "true"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::Identifier, "a"),
            Token::new(TokenType::LessEqual, "<="),
            Token::new(TokenType::Int, "4"),
            Token::new(TokenType::EqualEqual, "=="),
            Token::new(TokenType::False, "false"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::Identifier, "a"),
            Token::new(TokenType::BangEqual, "!="),
            Token::new(TokenType::Int, "5"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::Identifier, "a"),
            Token::new(TokenType::SlashEqual, "/="),
            Token::new(TokenType::Int, "2"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::Identifier, "a"),
            Token::new(TokenType::EqualEqual, "=="),
            Token::new(TokenType::Int, "5"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::Identifier, "a"),
            Token::new(TokenType::AsteriskEqual, "*="),
            Token::new(TokenType::Int, "1"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::Bang, "!"),
            Token::new(TokenType::LeftParen, "("),
            Token::new(TokenType::Identifier, "a"),
            Token::new(TokenType::EqualEqual, "=="),
            Token::new(TokenType::Int, "5"),
            Token::new(TokenType::RightParen, ")"),
            Token::new(TokenType::EqualEqual, "=="),
            Token::new(TokenType::False, "false"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::Eof, ""),
        ];
        let mut lexer = Lexer::new(input);
        for expected_token in expected_tokens {
            assert_eq!(lexer.next_token(), expected_token);
            println!("Passed {expected_token:?}");
        }
    }

    #[test]
    fn test_conditionals() {
        let input = r"
        let num = rand(0, 10);
        if (num > 8) {
            return true;
        } else {
            return false;
        }
        ";
        let expected_tokens = [
            Token::new(TokenType::Let, "let"),
            Token::new(TokenType::Identifier, "num"),
            Token::new(TokenType::Assign, "="),
            Token::new(TokenType::Identifier, "rand"),
            Token::new(TokenType::LeftParen, "("),
            Token::new(TokenType::Int, "0"),
            Token::new(TokenType::Comma, ","),
            Token::new(TokenType::Int, "10"),
            Token::new(TokenType::RightParen, ")"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::If, "if"),
            Token::new(TokenType::LeftParen, "("),
            Token::new(TokenType::Identifier, "num"),
            Token::new(TokenType::Greater, ">"),
            Token::new(TokenType::Int, "8"),
            Token::new(TokenType::RightParen, ")"),
            Token::new(TokenType::LeftBrace, "{"),
            Token::new(TokenType::Return, "return"),
            Token::new(TokenType::True, "true"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::RightBrace, "}"),
            Token::new(TokenType::Else, "else"),
            Token::new(TokenType::LeftBrace, "{"),
            Token::new(TokenType::Return, "return"),
            Token::new(TokenType::False, "false"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::RightBrace, "}"),
            Token::new(TokenType::Eof, ""),
        ];
        let mut lexer = Lexer::new(input);
        for expected_token in expected_tokens {
            assert_eq!(lexer.next_token(), expected_token);
            println!("Passed {expected_token:?}");
        }
    }
}
