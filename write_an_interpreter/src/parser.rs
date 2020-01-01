use crate::ast::{
    Expression, ExpressionStatement, Identifier, LetStatement, Program, ReturnStatement, Statement,
};
use crate::lexer::Operator;
use crate::lexer::{Delimiter, TokenType};
use crate::lexer::{Keyword, Lexer, Token};
use log::*;
use std::cell::{RefCell, RefMut};
use std::collections::HashMap;

pub trait PrefixParser<'a> {
    fn parse(&self, parser: RefMut<&mut Parser<'a>>) -> Box<dyn Expression>;
}

struct IdentifierParser {}

impl<'a> PrefixParser<'a> for IdentifierParser {
    fn parse(&self, mut parser: RefMut<&mut Parser<'a>>) -> Box<dyn Expression> {
        let ident = parser.next_token().unwrap();
        if let Token::Identifier(id) = ident {
            // TODO: Ignore things and find the next semilcolon.
            // This doesn't handle binary expressions like: a + b
            loop {
                let next_token = parser.next_token();
                if next_token.is_some()
                    && next_token.unwrap() == Token::Delimiter(Delimiter::Semicolon)
                {
                    break;
                }
            }
            Box::new(id)
        } else {
            panic!("Expected an identifier, but got {:?}", ident);
        }
    }
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    // The next token that is to be consumed
    peek_token: Option<Token>,
    prefix_parser_map: HashMap<TokenType, Box<dyn PrefixParser<'a>>>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Parser<'a> {
        let mut prefix_parser_map: HashMap<TokenType, Box<dyn PrefixParser<'a>>> = HashMap::new();
        prefix_parser_map.insert(TokenType::Identifier, Box::new(IdentifierParser {}));
        Parser {
            lexer,
            peek_token: None,
            prefix_parser_map,
        }
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program::new();
        loop {
            if self.peek_token().is_some() {
                let stmt = self.parse_statement();
                match stmt {
                    Some(stmt) => {
                        program.add_statement(stmt);
                    }
                    None => panic!("Failed to parse next statement!"),
                }
            } else {
                break;
            }
        }

        program
    }

    fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        match self.peek_token() {
            Some(Token::Keyword(Keyword::Let)) => Some(Box::new(self.parse_let_statement())),
            Some(Token::Keyword(Keyword::Return)) => Some(Box::new(self.parse_return_statement())),
            _ => Some(Box::new(self.parse_expression_statement())),
        }
    }

    fn parse_let_statement(&mut self) -> LetStatement {
        self.next_token(); // consume the let itself
        let identifier = match self.next_token() {
            Some(Token::Identifier(identifier)) => identifier,
            token => panic!("Expected an identifier after let but got {:?}", token),
        };

        if !self.expect_peek(Token::Operator(Operator::Assignment)) {
            panic!("Expected = after let but got {:?}", self.peek_token());
        }

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

    fn parse_return_statement(&mut self) -> ReturnStatement {
        self.next_token(); // consume the return itself

        // TODO: we're skipping the expressions until we encounter a semicolon
        loop {
            let next_token = self.next_token();
            if next_token.is_some() && next_token.unwrap() == Token::Delimiter(Delimiter::Semicolon)
            {
                break;
            }
        }
        debug!("Done parsing a return statement");

        ReturnStatement {}
    }

    fn parse_expression_statement(&mut self) -> ExpressionStatement {
        let expr = self.parse_expression();
        ExpressionStatement { expr }
    }

    fn parse_expression(&mut self) -> Box<dyn Expression> {
        let peek_token = self.peek_token();
        if peek_token.is_some() {
            let peek_token = peek_token.unwrap();
            match peek_token {
                Token::Identifier(identifier) => {
                    let rc = RefCell::new(self);
                    let parser = rc.borrow();
                    let parser = parser
                        .prefix_parser_map
                        .get(&TokenType::Identifier)
                        .unwrap();
                    parser.parse(rc.borrow_mut())
                }
                _ => panic!("No parser found for token: {:?}", peek_token),
            }
        } else {
            panic!("Can't parse expression because there's no more tokens!");
        }
    }

    fn expect_peek(&mut self, expected_token: Token) -> bool {
        match self.peek_token() {
            Some(token) => token == expected_token,
            None => false,
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        let token = self.peek_token_helper(true);
        debug!("Next token: {:?}", token);
        token
    }

    fn peek_token(&mut self) -> Option<Token> {
        let token = self.peek_token_helper(false);
        debug!("Peek token: {:?}", token);
        token
    }

    fn peek_token_helper(&mut self, consume: bool) -> Option<Token> {
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
        debug!("Peek token: {:?}", token);
        if consume {
            self.peek_token = None;
        }
        token
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
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
        println!("program: {}", program);
    }

    #[test]
    fn test_return_statements() {
        init();
        let input = r#"
return 5;
return 10;
return 838383;
        "#;

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        println!("program: {}", program);
    }

    #[test]
    fn test_identifier_expression() {
        init();
        let input = r#"
foobar;
        "#;

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        println!("program: {}", program);
    }
}
