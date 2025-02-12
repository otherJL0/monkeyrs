use crate::lexer;
use crate::token::{Token, TokenType};
use std::fmt;

#[derive(Debug)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

pub enum Expression {}

#[derive(Debug)]
pub enum Statement {
    LetStatement(LetStatement),
    ReturnStatement(ReturnStatement),
    ExpressionStatement(ExpressionStatement),
}

impl Node for Statement {
    fn token_literal(&self) -> String {
        match self {
            Statement::LetStatement(stmt) => stmt.token_literal(),
            Statement::ReturnStatement(stmt) => stmt.token_literal(),
            Statement::ExpressionStatement(stmt) => stmt.token_literal(),
        }
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = match self {
            Statement::LetStatement(stmt) => stmt.to_string(),
            Statement::ReturnStatement(stmt) => stmt.to_string(),
            Statement::ExpressionStatement(stmt) => stmt.to_string(),
        };
        write!(f, "{output}")
    }
}

#[derive(Debug)]
pub struct ExpressionStatement {
    token: Token,
    expression: Option<String>,
}

impl ExpressionStatement {
    pub fn new(token: Token) -> ExpressionStatement {
        ExpressionStatement {
            token,
            expression: None,
        }
    }
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl fmt::Display for ExpressionStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{};", self.expression.clone().unwrap())
    }
}

#[derive(Debug)]
pub struct ReturnStatement {
    token: Token,
    expression: Option<String>,
}

impl ReturnStatement {
    pub fn new(token: Token) -> ReturnStatement {
        ReturnStatement {
            token,
            expression: None,
        }
    }
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl fmt::Display for ReturnStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "return {};", self.expression.clone().unwrap())
    }
}

#[derive(Debug)]
pub struct LetStatement {
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

impl fmt::Display for LetStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "let {} = {}", &self.name.value, &self.value)
    }
}

pub trait Node {
    fn token_literal(&self) -> String;
}

pub struct Program {
    pub statements: Vec<Statement>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        self.statements
            .first()
            .map(|s| s.token_literal())
            .unwrap_or_default()
    }
}
