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
pub struct Let {
    pub token: token::Token,
    pub name: Identifier,
    pub value: Option<Expression>,
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
