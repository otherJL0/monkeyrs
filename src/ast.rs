use crate::token;
use std::fmt;

pub trait Node: fmt::Display + fmt::Debug {
    fn token_literal(&self) -> &str;
}

// pub trait Statement: Node {}
// pub trait Expression: Node + fmt::Debug {}
#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
    Boolean(Boolean),
    PrefixExpression(PrefixExpression),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Expression::Identifier(expr) => expr.to_string(),
                Expression::IntegerLiteral(integer) => integer.to_string(),
                Expression::Boolean(boolean) => boolean.to_string(),
                Expression::PrefixExpression(prefix_expression) => prefix_expression.to_string(),
            }
        )
    }
}

#[derive(Debug)]
pub enum Statement {
    LetStmt(LetStmt),
    ReturnStmt(ReturnStmt),
    ExpressionStmt(ExpressionStmt),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Statement::LetStmt(let_stmt) => let_stmt.to_string(),
                Statement::ReturnStmt(return_stmt) => return_stmt.to_string(),
                Statement::ExpressionStmt(expression_stmt) => expression_stmt.to_string(),
            }
        )
    }
}

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

#[derive(Debug, Clone)]
pub struct IntegerLiteral {
    pub token: token::Token,
    pub value: i64,
}

impl IntegerLiteral {
    pub fn new(token: &token::Token) -> IntegerLiteral {
        let value = token.literal.parse::<i64>().unwrap();
        IntegerLiteral {
            token: token.clone(),
            value,
        }
    }
}

impl fmt::Display for IntegerLiteral {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
impl Node for IntegerLiteral {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

#[derive(Debug, Clone)]
pub struct Boolean {
    pub token: token::Token,
    pub value: bool,
}

impl Boolean {
    pub fn new(token: &token::Token) -> Boolean {
        Boolean {
            token: token.clone(),
            value: match token.token_type {
                token::TokenType::True => true,
                token::TokenType::False => false,
                _ => unreachable!("Only True or False should be possible"),
            },
        }
    }
}

impl fmt::Display for Boolean {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
impl Node for Boolean {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

#[derive(Debug, Clone)]
pub struct PrefixExpression {
    token: token::Token,
    operator: String,
    right: Box<Expression>,
}

impl PrefixExpression {
    pub fn new(token: token::Token, expression: Box<Expression>) -> PrefixExpression {
        PrefixExpression {
            right: expression,
            token: token.clone(),
            operator: token.literal,
        }
    }
}

impl fmt::Display for PrefixExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}{})", self.operator, self.right.to_string())
    }
}

impl Node for PrefixExpression {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

#[derive(Debug)]
pub struct LetStmt {
    pub token: token::Token,
    pub name: Identifier,
    pub value: Option<Expression>,
}
impl LetStmt {
    pub fn new(identifier: Identifier, value: Option<Expression>) -> LetStmt {
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

#[derive(Debug)]
pub struct ExpressionStmt {
    pub token: token::Token,
    pub expression: Expression,
}

impl fmt::Display for ExpressionStmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.expression)
    }
}

impl Node for ExpressionStmt {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

#[derive(Default, Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
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
        if let Some(stmt) = self.statements.first() {
            match stmt {
                Statement::LetStmt(let_stmt) => &let_stmt.token,
                Statement::ReturnStmt(return_stmt) => &return_stmt.token,
                Statement::ExpressionStmt(expression_stmt) => &expression_stmt.token,
            }
            .literal
            .as_str()
        } else {
            ""
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_string() {
        let program = Program {
            statements: vec![Statement::LetStmt(LetStmt::new(
                Identifier::new("myVar"),
                Some(Expression::Identifier(Identifier::new("anotherVar"))),
            ))],
        };
        assert_eq!(program.to_string(), "let myVar = anotherVar;");
    }
}
