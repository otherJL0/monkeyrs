use crate::ast;
use crate::lexer;
use crate::token;

pub struct Parser<'a> {
    lexer: lexer::Lexer<'a>,
    current_token: token::Token,
    peek_token: token::Token,
    pub errors: Vec<String>,
}

impl<'a> Parser<'a> {
    fn new(mut lexer: lexer::Lexer<'a>) -> Parser<'a> {
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();
        Parser {
            lexer,
            current_token,
            peek_token,
            errors: vec![],
        }
    }

    fn expect_peek(&mut self, token_type: token::TokenType) -> bool {
        if self.peek_token.is_type(token_type.clone()) {
            self.advance();
            true
        } else {
            self.peek_error(token_type.clone());
            false
        }
    }
    fn peek_error(&mut self, token_type: token::TokenType) {
        let message = format!(
            "expected next token to be {:?}, got {:?} instead",
            token_type, self.peek_token.token_type
        );
        self.errors.push(message)
    }

    fn advance(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn parse_let_statement(&mut self) -> Option<ast::Statement> {
        if !self.peek_token.is_type(token::TokenType::Identifier) {
            return None;
        }
        let token = self.current_token.clone();
        if !self.expect_peek(token::TokenType::Identifier) {
            return None;
        }
        let name = ast::Identifier {
            token: self.current_token.clone(),
            value: self.current_token.literal.clone(),
        };
        if !self.expect_peek(token::TokenType::Assign) {
            return None;
        }
        while !self.current_token.is_type(token::TokenType::Semicolon) {
            self.advance();
        }
        let statement = ast::Let {
            token,
            name,
            value: None,
        };
        Some(ast::Statement::Let(statement))
    }

    fn parse_return_statement(&mut self) -> Option<ast::Statement> {
        let return_statement = ast::Return {
            token: self.current_token.clone(),
            return_value: None,
        };
        while !self.current_token.is_type(token::TokenType::Semicolon) {
            self.advance();
        }
        Some(ast::Statement::Return(return_statement))
    }

    fn parse_statement(&mut self) -> Option<ast::Statement> {
        match self.current_token.token_type {
            token::TokenType::Let => self.parse_let_statement(),
            token::TokenType::Return => self.parse_return_statement(),
            _ => None,
        }
    }

    fn parse_program(&mut self) -> Option<ast::Program> {
        let mut program = ast::Program::default();
        while self.current_token.token_type != token::TokenType::Eof {
            if let Some(statement) = self.parse_statement() {
                program.statements.push(statement);
            }
            self.advance();
        }
        Some(program)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_let_statements() {
        let input = "
        let x = 5;
        let y = 10;
        let foobar = 3141259265;
        ";
        let lexer = lexer::Lexer::new(&input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        assert!(
            parser.errors.is_empty(),
            "parser has {} errors:\n{}",
            parser.errors.len(),
            parser.errors.join("\n")
        );
        assert!(program.is_some(), "parser.parse_program returned None");
        assert_eq!(
            program.unwrap().statements.len(),
            3,
            "program.statements dows not contain three statements"
        );
    }

    #[test]
    fn test_return_statement() {
        let input = "
        return 10;
        return 8 + 9;
        return double(5);
        ";
        let lexer = lexer::Lexer::new(&input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        assert!(program.is_some(), "parser.parse_program returned None");
        assert!(
            program.unwrap().statements.len() == 3,
            "expected 3 statements"
        );
    }
}
