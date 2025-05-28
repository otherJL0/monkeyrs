use crate::token;
pub trait Node {
    fn token_literal(&self) -> String;
}

pub trait Statement: Node {}
pub trait Expression: Node {}

#[derive(Debug)]
pub struct Identifier {
    pub token: token::Token,
    pub value: String,
}

#[derive(Debug)]
pub struct LetStmt {
    pub token: token::Token,
    pub name: Identifier,
    pub value: Option<ExpressionStmt>,
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
impl Node for ReturnStmt {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}
impl Statement for ReturnStmt {}

#[derive(Debug)]
pub struct ExpressionStmt {
    token: token::Token,
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

impl Node for Program {
    fn token_literal(&self) -> String {
        if self.statements.is_empty() {
            String::from(" ")
        } else {
            self.statements.first().unwrap().token_literal()
        }
    }
}
