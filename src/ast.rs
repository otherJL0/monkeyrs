use crate::token;
pub trait Node {
    fn token_literal(&self) -> String;
}

#[derive(Debug)]
struct Identifier {
    token: token::Token,
    value: String,
}

#[derive(Debug)]
pub struct Let {
    token: token::Token,
    name: Identifier,
    value: Expression,
}

#[derive(Debug)]
pub struct Return {}

#[derive(Debug)]
pub enum Statement {
    Let(Let),
    Return(Return),
    Expression(Expression),
}

impl Node for Statement {
    fn token_literal(&self) -> String {
        String::from("Statement token literal")
    }
}

#[derive(Debug)]
pub enum Expression {}

pub struct Program {
    statements: Vec<Statement>,
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
