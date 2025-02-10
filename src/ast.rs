use crate::lexer;
use crate::token::{Token, TokenType};

#[derive(Debug)]
struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

#[derive(Debug)]
enum StatementType {
    LetStatement(LetStatement),
}

impl Node for StatementType {
    fn token_literal(&self) -> String {
        match self {
            StatementType::LetStatement(let_stmt) => let_stmt.token_literal(),
        }
    }
}

#[derive(Debug)]
struct LetStatement {
    token: Token,
    name: Identifier,
    value: String,
}

impl LetStatement {
    pub fn new(token: Token, name: Identifier) -> LetStatement {
        LetStatement {
            token,
            name,
            value: String::default(),
        }
    }
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

trait Node {
    fn token_literal(&self) -> String;
}

#[derive(Debug)]
struct Statement {}

impl Node for Statement {
    fn token_literal(&self) -> String {
        "Statement".to_string()
    }
}

struct Program {
    statements: Vec<StatementType>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        self.statements
            .first()
            .map(|s| s.token_literal())
            .unwrap_or_default()
    }
}

struct Parser<'a> {
    lexer: &'a mut lexer::Lexer<'a>,
    current_token: Token,
    next: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut lexer::Lexer<'a>) -> Parser<'a> {
        let current = lexer.next_token();
        let next_token = lexer.next_token();
        Parser {
            lexer,
            current_token: current,
            next: next_token,
        }
    }

    fn expect_peek(&mut self, token_type: TokenType) -> bool {
        if self.next.token_type == token_type {
            self.advance();
            true
        } else {
            false
        }
    }

    fn advance(&mut self) {
        self.current_token = self.next.clone();
        self.next = self.lexer.next_token().clone();
    }

    fn parse_let_statement(&mut self) -> Option<StatementType> {
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
        self.advance();
        Some(StatementType::LetStatement(stmt))
    }

    fn parse_statement(&mut self) -> Option<StatementType> {
        match self.current_token.token_type.clone() {
            TokenType::Let => self.parse_let_statement(),
            x => todo!("found type {:?}", x.clone()),
        }
    }

    pub fn parse_program(&mut self) -> Option<Program> {
        let mut statements = vec![];
        while self.current_token.token_type != TokenType::Eof {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            }
        }
        if statements.is_empty() {
            None
        } else {
            Some(Program { statements })
        }
    }
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
}
