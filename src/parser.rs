// use std::collections::HashMap;

use crate::ast::{
    Expression, ExpressionStatement, Identifier, LetStatement, Node, Program, ReturnStatement,
    Statement,
};
use crate::lexer;
use crate::token::{Token, TokenType};

enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}

struct Parser<'a> {
    lexer: &'a mut lexer::Lexer<'a>,
    current_token: Token,
    next: Token,
    errors: Vec<String>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut lexer::Lexer<'a>) -> Parser<'a> {
        let current = lexer.next_token();
        let next_token = lexer.next_token();
        Parser {
            lexer,
            current_token: current,
            next: next_token,
            errors: Vec::new(),
        }
    }

    fn match_infix_parse_fn(
        &self,
        token_type: TokenType,
        expression: Expression,
    ) -> Option<Expression> {
        todo!()
    }
    fn match_prefix_parse_fn(&self, token_type: &TokenType) -> Option<Expression> {
        match *token_type {
            TokenType::Identifier => Some(self.parse_identifier()),
            _ => None,
        }
    }
    fn parse_identifier(&self) -> Expression {
        Expression::Identifier(Identifier {
            token: self.current_token.clone(),
            value: self.current_token.literal.clone(),
        })
    }

    fn expect_peek(&mut self, token_type: TokenType) -> bool {
        if self.next.token_type == token_type {
            self.advance();
            true
        } else {
            let error = format!(
                "expected next token to be {:?}, got {:?} instead",
                token_type, self.next.token_type
            );
            self.errors.push(error);
            false
        }
    }

    fn advance(&mut self) {
        self.current_token = self.next.clone();
        self.next = self.lexer.next_token();
    }

    fn parse_expression(&self, _precedence: Precedence) -> Option<Expression> {
        self.match_prefix_parse_fn(&self.current_token.token_type)
            .clone()
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        self.parse_expression(Precedence::Lowest).map(|expression| {
            let stmt = ExpressionStatement::new(self.current_token.clone(), expression);
            if self.expect_peek(TokenType::Semicolon) {
                self.advance();
            }
            Statement::ExpressionStatement(stmt)
        })
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {
        let stmt = ReturnStatement::new(self.current_token.clone());
        self.advance();
        while self.current_token.token_type != TokenType::Semicolon {
            self.advance()
        }
        Some(Statement::ReturnStatement(stmt))
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        if !self.expect_peek(TokenType::Identifier) {
            return None;
        }
        let token = self.current_token.clone();
        let name = Identifier {
            token: self.next.clone(),
            value: self.next.literal.clone(),
        };
        let stmt = LetStatement::new(token, name);

        if !self.expect_peek(TokenType::Assign) {
            return None;
        }

        while self.current_token.token_type != TokenType::Semicolon {
            self.advance();
        }
        Some(Statement::LetStatement(stmt))
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token.token_type.clone() {
            TokenType::Let => self.parse_let_statement(),
            TokenType::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    pub fn parse_program(&mut self) -> Option<Program> {
        let mut statements = vec![];
        while self.current_token.token_type != TokenType::Eof {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            }
            self.advance();
        }
        if statements.is_empty() {
            None
        } else {
            Some(Program { statements })
        }
    }
    pub fn errors(&self) -> Vec<String> {
        self.errors.clone()
    }

    // fn register_prefix_fn(&mut self, token_type: TokenType, prefix_fn: PrefixParseFn) {
    //     self.prefix_parse_fns.insert(token_type, prefix_fn);
    // }
    //
    // fn register_infix_fn(&mut self, token_type: TokenType, infix_fn: InfixParseFn) {
    //     self.infix_parse_fns.insert(token_type, infix_fn);
    // }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_let_statements() {
        let input = r"
        let x = 5;
        let y = 10;
        let pi = 314159;
        ";
        let expected_identifiers = ["x", "y", "pi"];
        let mut lexer = lexer::Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);
        if let Some(program) = parser.parse_program() {
            assert_eq!(
                program.statements.len(),
                3,
                "Expected three statements, got {}",
                program.statements.len()
            );
            for (expected, actual) in expected_identifiers
                .into_iter()
                .zip(program.statements.into_iter())
            {
                assert_eq!(expected, &actual.token_literal());
            }
        } else {
            assert!(false, "`Parser::parse_program()` returned None");
        }
    }

    #[test]
    fn test_let_statements_with_error() {
        let input = r"
        let x = 5;
        let  = 314159;
        ";
        let expected_errors = ["expected next token to be Identifier, got Assign instead"];
        let mut lexer = lexer::Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);
        if let Some(_program) = parser.parse_program() {
            assert!(!parser.errors().is_empty());
            for (actual, expected) in parser.errors().iter().zip(expected_errors.iter()) {
                assert_eq!(actual, *expected);
            }
        } else {
            assert!(false, "`Parser::parse_program()` returned None");
        }
    }

    #[test]
    fn test_parse_return_statement() {
        let input = r"
        return true;
        return 10;
        return 1 + 4;
        ";
        let mut lexer = lexer::Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);
        if let Some(program) = parser.parse_program() {
            assert!(program.statements.len() == 3);
            for stmt in program.statements {
                assert_eq!(
                    stmt.token_literal(),
                    "return",
                    "return_stmt.token_literal not `return`, got {}",
                    stmt.token_literal()
                );
            }
        }
    }
}
