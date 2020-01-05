use crate::ast::{
    Expression, ExpressionStatement, IdentifierExpression, InfixExpression, LetStatement,
    LiteralIntegerExpression, PrefixExpression, Program, ReturnStatement, Statement,
};
use crate::lexer::{Delimiter, Precedence};
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
                debug!("Parsed one statement");
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
            _ => Some(Box::new(
                self.parse_expression_statement(Precedence::Lowest),
            )),
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

    fn parse_expression_statement(&mut self, precedence: Precedence) -> ExpressionStatement {
        let expr = self.parse_expression(precedence);
        ExpressionStatement { expr }
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Box<dyn Expression> {
        let peek_token = self.peek_token();
        if peek_token.is_some() {
            let peek_token = peek_token.unwrap();
            debug!("before parsing prefix expression");
            let mut left_expr = self.parse_prefix_expression(peek_token);
            debug!("done parsing prefix expression");
            while self.peek_token().is_some()
                && self.peek_token().unwrap() != Token::Delimiter(Delimiter::Semicolon)
                && precedence < self.peek_precedence()
            {
                debug!("going to parse an infix expr");
                let next_token = self.next_token().unwrap();
                match next_token {
                    Token::Operator(op) => {
                        if op == Operator::PlusSign
                            || op == Operator::MinusSign
                            || op == Operator::Asterisk
                            || op == Operator::Slash
                            || op == Operator::LessThan
                            || op == Operator::GreaterThan
                            || op == Operator::Equal
                            || op == Operator::NotEqual
                        {
                            left_expr = self.parse_infix_expression(left_expr, op);
                        }
                    }
                    _ => info!("Token isn't an infix operator: {:?}", next_token),
                }
            }

            if self.peek_token().is_some()
                && self.peek_token().unwrap() == Token::Delimiter(Delimiter::Semicolon)
            {
                self.next_token();
            }

            return left_expr;
        } else {
            panic!("Can't parse expression because there's no more tokens!");
        }
    }

    fn parse_infix_expression(
        &mut self,
        left_expr: Box<dyn Expression>,
        operator: Operator,
    ) -> Box<dyn Expression> {
        // TODO: use current precedence
        let right_expr = self.parse_expression(Precedence::from_token(Token::Operator(operator)));
        Box::new(InfixExpression {
            left_expr,
            operator,
            right_expr,
        })
    }

    fn parse_prefix_expression(&mut self, token: Token) -> Box<dyn Expression> {
        match token {
            Token::Identifier(_id) => Box::new(self.parse_identifier_expression()),
            Token::Literal(_litearl) => Box::new(self.parse_literal_expression()),
            _ => panic!(
                "Don't know how to parse prefix expression for token: {:?}",
                token
            ),
        }
    }

    fn parse_identifier_expression(&mut self) -> IdentifierExpression {
        let ident = self.next_token().unwrap();
        if let Token::Identifier(id) = ident {
            IdentifierExpression { identifier: id }
        } else {
            panic!("Expected an identifier, but got {:?}", ident);
        }
    }

    fn parse_literal_expression(&mut self) -> LiteralIntegerExpression {
        let token = self.next_token().unwrap();
        if let Token::Literal(Literal::Integer(int)) = token {
            LiteralIntegerExpression { literal: int }
        } else {
            panic!("Expected an literal, but got {:?}", token);
        }
    }

    fn parse_prefix_operator_expression(&mut self) -> PrefixExpression {
        let token = self.next_token().unwrap();
        if let Token::Operator(operator) = token {
            let expr = self.parse_expression(Precedence::Lowest);
            PrefixExpression { operator, expr }
        } else {
            panic!("Expected an operator, but got {:?}", token);
        }
    }

    //    fn parse_infix_operator_expression(&mut self) -> InfixExpression {
    //        let token = self.next_token().unwrap();
    //        if let Token::Operator(operator) = token {
    //            let expr = self.parse_expression();
    //            InfixExpression { operator, expr }
    //        } else {
    //            panic!("Expected an operator, but got {:?}", token);
    //        }
    //    }
    //
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

    fn peek_precedence(&mut self) -> Precedence {
        let token = self.peek_token_helper(false);
        Precedence::from_token(token.unwrap())
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
    use std::collections::HashMap;

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

    #[test]
    fn test_infix_operator_expression() {
        init();
        let input = r#"
    5 + 6 + 7;
    5 - 6 + 7;
    5 - 6 * 7;
    5 - 6 * 7 + 3 / 4;
            "#;

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        println!("Infix operator expression: {}", program);
    }

    #[test]
    fn test_infix_operator_expression2() {
        init();
        let mut map = HashMap::new();
        map.insert("5 + 6 + 7;", "((5 + 6) + 7)");
        map.insert("5 - 6 + 7;", "((5 - 6) + 7)");
        map.insert("5 - 6 * 7;", "(5 - (6 * 7))");
        map.insert("5 - 6 * 7 + 3 / 4;", "((5 - (6 * 7)) + (3 / 4))");
        map.insert("a + b * c + d / e - f;", "(((a + (b * c)) + (d / e)) - f)");
        map.insert(
            "3 + 4 * 5 == 3 * 1 + 4 * 5",
            "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
        );

        for (input, repr) in map {
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);
            let expr = parser.parse_expression(Precedence::Lowest);
            assert_eq!(repr, expr.to_string());
        }
    }
}
