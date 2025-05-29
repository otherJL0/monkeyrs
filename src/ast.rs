use crate::token;
use std::fmt;

pub trait Node: fmt::Display {
    fn token_literal(&self) -> String;
}

pub trait Statement: Node {}
pub trait Expression: Node {}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub token: token::Token,
    pub value: String,
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, Clone)]
pub struct LetStmt {
    pub token: token::Token,
    pub name: Identifier,
    pub value: Option<ExpressionStmt>,
}

impl fmt::Display for LetStmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} = {};",
            self.token.literal,
            self.name,
            self.value
                .clone()
                .map_or(String::default(), |value| value.to_string())
        )
    }
}
impl Node for LetStmt {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}
impl Statement for LetStmt {}

#[derive(Debug)]
pub struct ReturnStmt {
    pub token: token::Token,
    pub return_value: Option<ExpressionStmt>,
}
impl fmt::Display for ReturnStmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {};",
            self.token_literal(),
            self.return_value
                .clone()
                .map_or(String::default(), |value| value.to_string())
        )
    }
}
impl Node for ReturnStmt {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}
impl Statement for ReturnStmt {}

#[derive(Debug, Clone)]
pub struct ExpressionStmt {
    token: token::Token,
}

impl fmt::Display for ExpressionStmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

impl Node for ExpressionStmt {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}
impl Statement for ExpressionStmt {}

#[derive(Default)]
pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}
impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if self.statements.is_empty() {
            String::from(" ")
        } else {
            self.statements.first().unwrap().token_literal()
        }
    }
}
