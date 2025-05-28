use crate::ast;
use crate::lexer;
use crate::token;

pub struct Parser<'a> {
    lexer: lexer::Lexer<'a>,

    current_token: token::Token,
    peek_token: token::Token,
}

impl<'a> Parser<'a> {
    fn new(mut lexer: lexer::Lexer<'a>) -> Parser<'a> {
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();
        Parser {
            lexer,
            current_token,
            peek_token,
        }
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn parse_let_statement(&mut self) -> Option<ast::Statement> {
        if !self.peek_token.is_type(token::TokenType::Identifier) {
            return None;
        }
        let token = self.current_token.clone();
        self.next_token();
        let name = ast::Identifier {
            token: self.current_token.clone(),
            value: self.current_token.literal.clone(),
        };
        if !self.peek_token.is_type(token::TokenType::Assign) {
            return None;
        }
        self.next_token();
        while !self.current_token.is_type(token::TokenType::Semicolon) {
            self.next_token();
        }
        let statement = ast::Let {
            token,
            name,
            value: None,
        };
        Some(ast::Statement::Let(statement))
    }

    fn parse_statement(&mut self) -> Option<ast::Statement> {
        match self.current_token.token_type {
            token::TokenType::Let => self.parse_let_statement(),
            _ => None,
        }
    }

    fn parse_program(&mut self) -> Option<ast::Program> {
        let mut program = ast::Program::default();
        while self.current_token.token_type != token::TokenType::Eof {
            if let Some(statement) = self.parse_statement() {
                program.statements.push(statement);
            }
            self.next_token();
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
        "
        .to_string();
        let lexer = lexer::Lexer::new(&input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        assert!(program.is_some(), "parser.parse_program returned None");
        assert_eq!(
            program.unwrap().statements.len(),
            3,
            "program.statements dows not contain three statements"
        );
    }
}
