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

    fn read_number(&mut self) -> token::Token {
        let start = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        let literal = &self.input[start..self.position];
        token::Token::new(token::TokenType::Int, literal.to_string())
    }
    fn read_identifier(&mut self) -> token::Token {
        let start = self.position;
        while self.ch.is_ascii_alphabetic() {
            self.read_char();
        }
        let literal = &self.input[start..self.position];
        let token_type = match literal {
            "fn" => token::TokenType::Function,
            "let" => token::TokenType::Let,
            _ => token::TokenType::Ident,
        };
        token::Token::new(token_type, String::from(literal))
    }

    pub fn next_token(&mut self) -> token::Token {
        while self.ch.is_whitespace() {
            self.read_char();
        }
        let token = match self.ch {
            '=' => token::Token::new(token::TokenType::Assign, String::from("=")),
            ';' => token::Token::new(token::TokenType::Semicolon, String::from(";")),
            '(' => token::Token::new(token::TokenType::LeftParen, String::from("(")),
            ')' => token::Token::new(token::TokenType::RightParen, String::from(")")),
            ',' => token::Token::new(token::TokenType::Comma, String::from(",")),
            '+' => token::Token::new(token::TokenType::Plus, String::from("+")),
            '{' => token::Token::new(token::TokenType::LeftBrace, String::from("{")),
            '}' => token::Token::new(token::TokenType::RightBrace, String::from("}")),
            '\0' => token::Token::new(token::TokenType::Eof, String::default()),
            _ => {
                if self.ch.is_ascii_alphabetic() {
                    return self.read_identifier();
                }
                if self.ch.is_ascii_digit() {
                    return self.read_number();
                }
                return token::Token::new(token::TokenType::Illegal, String::from(self.ch));
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
            token::Token::new(token::TokenType::Assign, String::from("=")),
            token::Token::new(token::TokenType::Plus, String::from("+")),
            token::Token::new(token::TokenType::LeftParen, String::from("(")),
            token::Token::new(token::TokenType::RightParen, String::from(")")),
            token::Token::new(token::TokenType::LeftBrace, String::from("{")),
            token::Token::new(token::TokenType::RightBrace, String::from("}")),
            token::Token::new(token::TokenType::Comma, String::from(",")),
            token::Token::new(token::TokenType::Semicolon, String::from(";")),
            token::Token::new(token::TokenType::Eof, String::from("")),
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
            token::Token::new(token::TokenType::Let, String::from("let")),
            token::Token::new(token::TokenType::Ident, String::from("five")),
            token::Token::new(token::TokenType::Assign, String::from("=")),
            token::Token::new(token::TokenType::Int, String::from("5")),
            token::Token::new(token::TokenType::Semicolon, String::from(";")),
            token::Token::new(token::TokenType::Let, String::from("let")),
            token::Token::new(token::TokenType::Ident, String::from("ten")),
            token::Token::new(token::TokenType::Assign, String::from("=")),
            token::Token::new(token::TokenType::Int, String::from("10")),
            token::Token::new(token::TokenType::Semicolon, String::from(";")),
            token::Token::new(token::TokenType::Let, String::from("let")),
            token::Token::new(token::TokenType::Ident, String::from("add")),
            token::Token::new(token::TokenType::Assign, String::from("=")),
            token::Token::new(token::TokenType::Function, String::from("fn")),
            token::Token::new(token::TokenType::LeftParen, String::from("(")),
            token::Token::new(token::TokenType::Ident, String::from("x")),
            token::Token::new(token::TokenType::Comma, String::from(",")),
            token::Token::new(token::TokenType::Ident, String::from("y")),
            token::Token::new(token::TokenType::RightParen, String::from(")")),
            token::Token::new(token::TokenType::LeftBrace, String::from("{")),
            token::Token::new(token::TokenType::Ident, String::from("x")),
            token::Token::new(token::TokenType::Plus, String::from("+")),
            token::Token::new(token::TokenType::Ident, String::from("y")),
            token::Token::new(token::TokenType::Semicolon, String::from(";")),
            token::Token::new(token::TokenType::RightBrace, String::from("}")),
            token::Token::new(token::TokenType::Semicolon, String::from(";")),
            token::Token::new(token::TokenType::Let, String::from("let")),
            token::Token::new(token::TokenType::Ident, String::from("result")),
            token::Token::new(token::TokenType::Assign, String::from("=")),
            token::Token::new(token::TokenType::Ident, String::from("add")),
            token::Token::new(token::TokenType::LeftParen, String::from("(")),
            token::Token::new(token::TokenType::Ident, String::from("five")),
            token::Token::new(token::TokenType::Comma, String::from(",")),
            token::Token::new(token::TokenType::Ident, String::from("ten")),
            token::Token::new(token::TokenType::RightParen, String::from(")")),
            token::Token::new(token::TokenType::Semicolon, String::from(";")),
            token::Token::new(token::TokenType::Eof, String::from("")),
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
