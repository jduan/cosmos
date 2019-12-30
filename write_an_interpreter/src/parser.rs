use crate::lexer::{Lexer, Token};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    next_token: Option<Token>,
    peek_token: Option<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Parser<'a> {
        Parser {
            lexer,
            next_token: None,
            peek_token: None,
        }
    }

    fn peek_token(&mut self) -> &Option<Token> {
        if self.peek_token.is_some() {
            &self.peek_token
        } else {
            let next_token = self.lexer.next_token();
            if next_token.is_eof() {
                self.peek_token = None;
            } else {
                self.peek_token = Some(next_token);
            }

            &self.peek_token
        }
    }

    fn next_token(&mut self) -> &Option<Token> {
        let token = self.peek_token();
        self.peek_token = None;
        token
    }

    //    pub fn parse_program() -> Program {}
}
