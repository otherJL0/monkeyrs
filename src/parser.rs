use std::collections::HashMap;

use crate::ast;
use crate::lexer;
use crate::token::{Token, TokenType};

enum Precedence {
    Lowest = 1,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}

enum PrefixParser {
    Identifier,
    Integer,
    Boolean,
    Prefix,
    Grouped,
    If,
    Function,
}

enum InfixParser {
    Plus,
    Minus,
    Multiply,
    Divide,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    Call,
}

pub struct Parser<'a> {
    lexer: lexer::Lexer<'a>,
    current_token: Token,
    peek_token: Token,
    pub errors: Vec<String>,
    prefix_parse_fns: HashMap<TokenType, PrefixParser>,
    infix_parse_fns: HashMap<TokenType, InfixParser>,
    precedences: HashMap<TokenType, Precedence>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Parser<'a> {
        let mut lexer = lexer::Lexer::new(input);
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();
        let prefix_parse_fns = HashMap::from([
            (TokenType::Identifier, PrefixParser::Identifier),
            (TokenType::Int, PrefixParser::Integer),
            (TokenType::True, PrefixParser::Boolean),
            (TokenType::False, PrefixParser::Boolean),
            (TokenType::Bang, PrefixParser::Prefix),
            (TokenType::Minus, PrefixParser::Prefix),
            (TokenType::Minus, PrefixParser::Prefix),
            (TokenType::Minus, PrefixParser::Prefix),
            (TokenType::Minus, PrefixParser::Prefix),
        ]);
        let infix_parse_fns = HashMap::from([
            (TokenType::Plus, InfixParser::Plus),
            (TokenType::Minus, InfixParser::Minus),
            (TokenType::Asterisk, InfixParser::Multiply),
            (TokenType::Slash, InfixParser::Divide),
            (TokenType::EqualEqual, InfixParser::Equal),
            (TokenType::BangEqual, InfixParser::NotEqual),
            (TokenType::Less, InfixParser::LessThan),
            (TokenType::Greater, InfixParser::GreaterThan),
            (TokenType::LeftParen, InfixParser::Call),
        ]);

        let precedences = HashMap::from([
            (TokenType::EqualEqual, Precedence::Equals),
            (TokenType::BangEqual, Precedence::Equals),
            (TokenType::Less, Precedence::LessGreater),
            (TokenType::Greater, Precedence::LessGreater),
            (TokenType::Plus, Precedence::Sum),
            (TokenType::Minus, Precedence::Sum),
            (TokenType::Slash, Precedence::Product),
            (TokenType::Asterisk, Precedence::Product),
            (TokenType::LeftParen, Precedence::Call),
        ]);
        Parser {
            lexer,
            current_token,
            peek_token,
            errors: vec![],
            prefix_parse_fns,
            infix_parse_fns,
            precedences,
        }
    }

    fn parse_prefix(&mut self, parser_type: PrefixParser) -> ast::Expression {
        match parser_type {
            PrefixParser::Identifier => self.parse_identifier(),
            PrefixParser::Integer => self.parse_integer(),
            PrefixParser::Boolean => self.parse_boolean(),
            PrefixParser::Prefix => self.parse_prefix_expression(),
            PrefixParser::Grouped => self.parse_grouped_expression(),
            PrefixParser::If => self.parse_if_expression(),
            PrefixParser::Function => self.parse_function(),
        }
    }

    fn parse_identifier(&mut self) -> ast::Expression {
        ast::Expression::Identifier(ast::Identifier::new(&self.current_token.literal))
    }

    fn parse_integer(&mut self) -> ast::Expression {
        todo!()
    }
    fn parse_boolean(&mut self) -> ast::Expression {
        todo!()
    }
    fn parse_prefix_expression(&mut self) -> ast::Expression {
        todo!()
    }
    fn parse_grouped_expression(&mut self) -> ast::Expression {
        todo!()
    }
    fn parse_if_expression(&mut self) -> ast::Expression {
        todo!()
    }
    fn parse_function(&mut self) -> ast::Expression {
        todo!()
    }

    fn expect_peek(&mut self, token_type: TokenType) -> bool {
        if self.peek_token.is_type(token_type) {
            self.advance();
            true
        } else {
            self.peek_error(token_type);
            false
        }
    }
    fn peek_error(&mut self, token_type: TokenType) {
        let message = format!(
            "expected next token to be {:?}, got {:?} instead",
            token_type, self.peek_token.token_type
        );
        self.errors.push(message);
    }

    fn advance(&mut self) {
        self.current_token = std::mem::replace(&mut self.peek_token, self.lexer.next_token());
    }

    fn parse_let_statement(&mut self) -> Option<ast::LetStmt> {
        if !self.peek_token.is_type(TokenType::Identifier) {
            return None;
        }
        if !self.expect_peek(TokenType::Identifier) {
            return None;
        }
        let name = ast::Identifier::new(&self.current_token.literal);
        if !self.expect_peek(TokenType::Assign) {
            return None;
        }
        while !self.current_token.is_type(TokenType::Semicolon) {
            self.advance();
        }
        let statement = ast::LetStmt::new(name, None);
        Some(statement)
    }

    fn parse_return_statement(&mut self) -> Option<ast::ReturnStmt> {
        let return_statement = ast::ReturnStmt::new(None);
        while !self.current_token.is_type(TokenType::Semicolon) {
            self.advance();
        }
        Some(return_statement)
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<ast::Expression> {
        None
    }

    fn parse_expression_statement(&mut self) -> Option<ast::ExpressionStmt> {
        let expression = self.parse_expression(Precedence::Lowest);
        if self.expect_peek(TokenType::Semicolon) {
            self.advance();
        }
        None
    }

    fn parse_statement(&mut self) -> Option<ast::Statement> {
        match self.current_token.token_type {
            TokenType::Let => self
                .parse_let_statement()
                .map(|stmt| ast::Statement::LetStmt(stmt)),
            TokenType::Return => self
                .parse_return_statement()
                .map(|stmt| ast::Statement::ReturnStmt(stmt)),
            _ => self
                .parse_expression_statement()
                .map(|stmt| ast::Statement::ExpressionStmt(stmt)),
        }
    }

    pub fn parse_program(&mut self) -> Option<ast::Program> {
        let mut program = ast::Program::default();
        while self.current_token.token_type != TokenType::Eof {
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

    #[test]
    fn test_identifier_statement() {
        let input = "foobar;";
        let mut parser = Parser::new(input);
        let program = parser.parse_program().unwrap();
        let stmt = program.statements.first().unwrap();
    }
}
