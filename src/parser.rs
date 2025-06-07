use std::collections::HashMap;

use crate::ast;
use crate::lexer;
use crate::token;

pub type prefix_parse_fn = fn() -> Box<dyn ast::Expression>;
pub type infix_parse_fn = fn(Box<dyn ast::Expression>) -> Box<ast::Expression>;

#[derive(Default)]
pub struct Parser<'a> {
    lexer: lexer::Lexer<'a>,
    current_token: token::Token,
    peek_token: token::Token,
    pub errors: Vec<String>,
    prefix_parse_fns: HashMap<token::TokenType, prefix_parse_fn>,
    infix_parse_fns: HashMap<token::TokenType, infix_parse_fn>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Parser<'a> {
        let mut lexer = lexer::Lexer::new(input);
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();
        Parser {
            lexer,
            current_token,
            peek_token,
            errors: vec![],
            prefix_parse_fns: HashMap::new(),
            infix_parse_fns: HashMap::new(),
        }
    }
    fn register_prefix(&mut self, token_type: token::TokenType, function: prefix_parse_fn) {
        self.prefix_parse_fns.insert(token_type, function);
    }

    fn register_infix(&mut self, token_type: token::TokenType, function: infix_parse_fn) {
        self.infix_parse_fns.insert(token_type, function);
    }

    fn expect_peek(&mut self, token_type: token::TokenType) -> bool {
        if self.peek_token.is_type(token_type) {
            self.advance();
            true
        } else {
            self.peek_error(token_type);
            false
        }
    }
    fn peek_error(&mut self, token_type: token::TokenType) {
        let message = format!(
            "expected next token to be {:?}, got {:?} instead",
            token_type, self.peek_token.token_type
        );
        self.errors.push(message);
    }

    fn advance(&mut self) {
        self.current_token = std::mem::replace(&mut self.peek_token, self.lexer.next_token());
    }

    fn parse_let_statement(&mut self) -> Option<Box<dyn ast::Statement>> {
        if !self.peek_token.is_type(token::TokenType::Identifier) {
            return None;
        }
        if !self.expect_peek(token::TokenType::Identifier) {
            return None;
        }
        let name = ast::Identifier::new(&self.current_token.literal);
        if !self.expect_peek(token::TokenType::Assign) {
            return None;
        }
        while !self.current_token.is_type(token::TokenType::Semicolon) {
            self.advance();
        }
        let statement = ast::LetStmt::new(name, None);
        Some(Box::new(statement))
    }

    fn parse_return_statement(&mut self) -> Option<Box<dyn ast::Statement>> {
        let return_statement = ast::ReturnStmt::new(None);
        while !self.current_token.is_type(token::TokenType::Semicolon) {
            self.advance();
        }
        Some(Box::new(return_statement))
    }

    fn parse_statement(&mut self) -> Option<Box<dyn ast::Statement>> {
        match self.current_token.token_type {
            token::TokenType::Let => self.parse_let_statement(),
            token::TokenType::Return => self.parse_return_statement(),
            _ => None,
        }
    }

    pub fn parse_program(&mut self) -> Option<ast::Program> {
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
        let mut parser = Parser::new(input);
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
        let mut parser = Parser::new(input);
        let program = parser.parse_program();
        assert!(program.is_some(), "parser.parse_program returned None");
        assert!(
            program.unwrap().statements.len() == 3,
            "expected 3 statements"
        );
    }
}
