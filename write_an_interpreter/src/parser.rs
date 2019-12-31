use crate::ast::{LetStatement, Program, Statement};
use crate::lexer::Delimiter;
use crate::lexer::Operator;
use crate::lexer::{Keyword, Lexer, Token};
use log::*;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    peek_token: Option<Token>,
    next_token_called: bool,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Parser<'a> {
        Parser {
            lexer,
            peek_token: None,
            next_token_called: false,
        }
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program::new();
        loop {
            if self.peek_token().is_some() {
                let stmt = self.parse_statement();
                match stmt {
                    Some(stmt) => program.add_statement(stmt),
                    None => panic!("Failed to parse next statement!"),
                }
            } else {
                break;
            }
        }

        program
    }

    fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        match self.next_token() {
            Some(Token::Keyword(Keyword::Let)) => Some(Box::new(self.parse_let_statement())),
            _ => None,
        }
    }

    fn parse_let_statement(&mut self) -> LetStatement {
        println!("Before parsing an identifier");
        let identifier = match self.next_token() {
            Some(Token::Identifier(identifier)) => identifier,
            token => panic!("Expected an identifier after let but got {:?}", token),
        };
        println!("Found expected identifier {:?}", identifier);

        if !self.expect_peek(Token::Operator(Operator::Assignment)) {
            panic!("Expected = after let but got {:?}", self.peek_token());
        }
        debug!("Found expected assignment =");

        // TODO: we're skipping the expressions until we encounter a semicolon
        loop {
            let next_token = self.next_token();
            if next_token.is_some() && next_token.unwrap() == Token::Delimiter(Delimiter::Semicolon)
            {
                break;
            }
        }
        debug!("Done parsing a let statement");

        LetStatement {
            name: identifier, //            value: Expression,
        }
    }

    fn expect_peek(&mut self, expected_token: Token) -> bool {
        match self.peek_token() {
            Some(token) => token == expected_token,
            None => false,
        }
    }

    fn peek_token(&mut self) -> Option<Token> {
        let token = if self.peek_token.is_some() {
            self.peek_token.clone()
        } else {
            let next_token = self.lexer.next_token();
            if next_token.is_eof() {
                self.peek_token = None;
            } else {
                self.peek_token = Some(next_token);
            }

            self.peek_token.clone()
        };
        println!("Peek token: {:?}", token);
        if self.next_token_called {
            self.peek_token = None;
            self.next_token_called = false;
        }
        token
    }

    fn next_token(&mut self) -> Option<Token> {
        self.next_token_called = true;
        let token = self.peek_token();
        println!("Next token: {:?}", token);
        token
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        env_logger::builder().is_test(true).try_init().unwrap();
    }

    #[test]
    fn test_let_statements() {
        init();
        let input = r#"
let x = 5;
let y = 10;
let foobar = 838383;
        "#;

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        println!("program: {:?}", program);
    }
}
