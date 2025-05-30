use crate::token;
use std::fmt;

pub trait Node: fmt::Display {
    fn token_literal(&self) -> &str;
}

pub trait Statement: Node {}
pub trait Expression: Node + fmt::Debug {}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub token: token::Token,
    pub value: String,
}

impl Identifier {
    pub fn new(value: &str) -> Identifier {
        Identifier {
            token: token::Token {
                token_type: token::TokenType::Identifier,
                literal: value.to_string(),
            },
            value: value.to_string(),
        }
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
impl Node for Identifier {
    fn token_literal(&self) -> &str {
        &self.value
    }
}
impl Expression for Identifier {}

#[derive(Debug)]
pub struct LetStmt {
    pub token: token::Token,
    pub name: Identifier,
    pub value: Option<Box<dyn Expression>>,
}
impl LetStmt {
    pub fn new(identifier: Identifier, value: Option<Box<dyn Expression>>) -> LetStmt {
        LetStmt {
            token: token::Token {
                token_type: token::TokenType::Let,
                literal: "let".to_string(),
            },
            name: identifier,
            value,
        }
    }
}

impl fmt::Display for LetStmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} = {};",
            self.token.literal,
            self.name,
            self.value
                .as_ref()
                .map_or(String::default(), std::string::ToString::to_string)
        )
    }
}
impl Node for LetStmt {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
}
impl Statement for LetStmt {}

#[derive(Debug)]
pub struct ReturnStmt {
    pub token: token::Token,
    pub return_value: Option<ExpressionStmt>,
}

impl ReturnStmt {
    pub fn new(return_value: Option<ExpressionStmt>) -> ReturnStmt {
        ReturnStmt {
            token: token::Token {
                token_type: token::TokenType::Return,
                literal: "return".to_string(),
            },
            return_value,
        }
    }
}
impl fmt::Display for ReturnStmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {};",
            self.token_literal(),
            self.return_value
                .as_ref()
                .map_or(String::default(), std::string::ToString::to_string)
        )
    }
}
impl Node for ReturnStmt {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
}
impl Statement for ReturnStmt {}

#[derive(Debug)]
pub struct ExpressionStmt {
    token: token::Token,
    expression: Box<dyn Expression>,
}

impl fmt::Display for ExpressionStmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.expression.as_ref())
    }
}

impl Node for ExpressionStmt {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
}
impl Statement for ExpressionStmt {}

#[derive(Default)]
pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}
impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut statements = vec![];
        for statement in &self.statements {
            statements.push(statement.to_string());
        }
        write!(f, "{}", statements.join("\n"))
    }
}

impl Node for Program {
    fn token_literal(&self) -> &str {
        if self.statements.is_empty() {
            ""
        } else {
            self.statements.first().unwrap().token_literal()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_string() {
        let program = Program {
            statements: vec![Box::new(LetStmt::new(
                Identifier::new("myVar"),
                Some(Box::new(Identifier::new("anotherVar"))),
            ))],
        };
        assert_eq!(program.to_string(), "let myVar = anotherVar;");
    }
}
