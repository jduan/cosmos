use crate::ast::{
    Expression, ExpressionStatement, IdentifierExpression, LetStatement, LiteralIntegerExpression,
    PrefixExpression, Program, ReturnStatement, Statement,
};
use crate::lexer::{Delimiter, TokenType};
use crate::lexer::{Keyword, Lexer, Token};
use crate::lexer::{Literal, Operator};
use log::*;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    // The next token that is to be consumed
    peek_token: Option<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Parser<'a> {
        Parser {
            lexer,
            peek_token: None,
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
            match peek_token.get_token_type() {
                TokenType::Identifier => Box::new(self.parse_identifier_expression()),
                TokenType::Literal => Box::new(self.parse_literal_expression()),
                TokenType::Operator => Box::new(self.parse_prefix_operator_expression()),
                _ => panic!("No parser found for token: {:?}", peek_token),
            }
        } else {
            panic!("Can't parse expression because there's no more tokens!");
        }
    }

    fn parse_identifier_expression(&mut self) -> IdentifierExpression {
        let ident = self.next_token().unwrap();
        if let Token::Identifier(id) = ident {
            // TODO: Ignore things and find the next semilcolon.
            // This doesn't handle binary expressions like: a + b
            loop {
                let next_token = self.next_token();
                if next_token.is_some()
                    && next_token.unwrap() == Token::Delimiter(Delimiter::Semicolon)
                {
                    break;
                }
            }
            IdentifierExpression { identifier: id }
        } else {
            panic!("Expected an identifier, but got {:?}", ident);
        }
    }

    fn parse_literal_expression(&mut self) -> LiteralIntegerExpression {
        let token = self.next_token().unwrap();
        if let Token::Literal(Literal::Integer(int)) = token {
            // TODO: Ignore things and find the next semilcolon.
            // This doesn't handle binary expressions like: a + b
            loop {
                let next_token = self.next_token();
                if next_token.is_some()
                    && next_token.unwrap() == Token::Delimiter(Delimiter::Semicolon)
                {
                    break;
                }
            }
            LiteralIntegerExpression { literal: int }
        } else {
            panic!("Expected an literal, but got {:?}", token);
        }
    }

    fn parse_prefix_operator_expression(&mut self) -> PrefixExpression {
        let token = self.next_token().unwrap();
        if let Token::Operator(operator) = token {
            let expr = self.parse_expression();
            PrefixExpression { operator, expr }
        } else {
            panic!("Expected an operator, but got {:?}", token);
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
    fn full_test() {
        init();
        let input = r#"
let x = 5;
return 5;
foobar;
-50;
5;
        "#;

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        println!("{}", program);
    }

    #[test]
    fn test_let_statements() {
        init();
        let input = r#"
let x = 5;
        "#;

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let stmt = parser.parse_let_statement();
        assert_eq!(
            LetStatement {
                name: String::from("x")
            },
            stmt
        );
    }

    #[test]
    fn test_return_statements() {
        init();
        let input = r#"
return 5;
        "#;

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let stmt = parser.parse_return_statement();
        assert_eq!(ReturnStatement {}, stmt);
    }

    #[test]
    fn test_identifier_expression() {
        init();
        let input = r#"
foobar;
        "#;

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let expr = parser.parse_identifier_expression();
        assert_eq!(
            IdentifierExpression {
                identifier: String::from("foobar")
            },
            expr
        );
    }

    #[test]
    fn test_literal_expression() {
        init();
        let input = r#"
50;
        "#;

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let expr = parser.parse_literal_expression();
        assert_eq!(LiteralIntegerExpression { literal: 50 }, expr);
    }

    #[test]
    fn test_prefix_operator_expression() {
        init();
        let input = r#"
!50;
        "#;

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let expr = parser.parse_prefix_operator_expression();
        println!("Prefix operator expression: {}", expr);
    }
}
