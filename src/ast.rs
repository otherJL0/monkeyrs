use crate::token;
pub trait Node {
    fn token_literal(&self) -> String;
}

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

#[derive(Debug)]
pub struct ReturnStmt {
    pub token: token::Token,
    pub return_value: Option<ExpressionStmt>,
}

#[derive(Debug)]
pub enum Statement {
    Let(LetStmt),
    Return(ReturnStmt),
    Expression(ExpressionStmt),
}

impl Node for Statement {
    fn token_literal(&self) -> String {
        String::from("Statement token literal")
    }
}

#[derive(Debug)]
pub struct ExpressionStmt {
    token: token::Token,
    expression: Expression,
}

#[derive(Debug)]
pub struct Expression {}

#[derive(Default)]
pub struct Program {
    pub statements: Vec<Statement>,
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
