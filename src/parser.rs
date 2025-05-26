use crate::ast;
use crate::lexer;
use crate::token;

pub struct Parser<'a> {
    lexer: lexer::Lexer<'a>,

    current_token: token::Token,
    peek_token: token::Token,
}

impl<'a> Parser<'a> {
    fn new(mut lexer: lexer::Lexer<'a>) -> Parser<'a> {
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();
        Parser {
            lexer,
            current_token,
            peek_token,
        }
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn parse_program(&self) {}
}
