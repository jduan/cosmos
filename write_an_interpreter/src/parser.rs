use crate::ast::{
    BangUnaryExpression, BoolLiteralExpression, CallExpression, Expression, ExpressionStatement,
    GroupedExpression, IdentifierExpression, InfixExpression, LetStatement,
    LiteralIntegerExpression, MinusUnaryExpression, Program, ReturnStatement, Statement,
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

        self.expect_next(Token::Operator(Operator::Assignment));
        let expr = self.parse_expression(Precedence::Lowest);
        debug!("Done parsing a let statement");
        self.consume_semicolon();

        LetStatement {
            name: identifier,
            expr,
        }
    }

    fn parse_return_statement(&mut self) -> ReturnStatement {
        self.next_token(); // consume the return itself
        let expr = self.parse_expression(Precedence::Lowest);
        self.consume_semicolon();
        debug!("Done parsing a return statement");

        ReturnStatement { expr }
    }

    fn parse_expression_statement(&mut self, precedence: Precedence) -> ExpressionStatement {
        let expr = self.parse_expression(precedence);
        debug!("Done parsing an expression statement");
        ExpressionStatement { expr }
    }

    /// Parse a list of expressions, such as when calling a function. The list can be empty.
    fn parse_expressions(&mut self, precedence: Precedence) -> Vec<Box<dyn Expression>> {
        let peek_token = self.peek_token();
        if peek_token.is_none() {
            panic!("Expected more tokens when parsing a list of expressions but got a None");
        }

        return if peek_token == Some(Token::Delimiter(Delimiter::RightParen)) {
            self.next_token();
            vec![]
        } else {
            let expr = self.parse_expression(precedence);
            //            add(b * c)
            debug!("Parsed one argument of a function call: {}", expr);
            // Consume the next comma if there's one
            if let Some(Token::Delimiter(Delimiter::Comma)) = self.peek_token() {
                self.next_token();
            }
            let mut rest_exprs = self.parse_expressions(precedence);
            rest_exprs.push(expr);
            rest_exprs
        };
    }

    /// This is the core of the parser. It uses the Pratt parser.
    /// See https://en.wikipedia.org/wiki/Pratt_parser
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
            self.consume_semicolon();

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
            Token::Delimiter(Delimiter::LeftParen) => Box::new(self.parse_grouped_expression()),
            Token::Keyword(Keyword::True) => Box::new(self.parse_bool_literal_expression(true)),
            Token::Keyword(Keyword::False) => Box::new(self.parse_bool_literal_expression(false)),
            Token::Identifier(_id) => self.parse_identifier_expression(),
            Token::Literal(_litearl) => Box::new(self.parse_literal_expression()),
            Token::Operator(Operator::MinusSign) => Box::new(self.parse_minus_unary_expression()),
            Token::Operator(Operator::Bang) => Box::new(self.parse_bang_unary_expression()),
            _ => panic!(
                "Don't know how to parse prefix expression for token: {:?}",
                token
            ),
        }
    }

    fn parse_grouped_expression(&mut self) -> GroupedExpression {
        self.next_token();
        let expr = self.parse_expression(Precedence::Lowest);
        debug!("Parsed a grouped expression: {}", expr);
        let next_token = self.next_token();
        if let Some(Token::Delimiter(Delimiter::RightParen)) = next_token {
            GroupedExpression { expr }
        } else {
            panic!(
                "Expect a closing parent ) for a grouped expression but got: {:?}",
                next_token
            );
        }
    }

    fn parse_bool_literal_expression(&mut self, b: bool) -> BoolLiteralExpression {
        self.next_token();
        BoolLiteralExpression { literal: b }
    }

    fn parse_identifier_expression(&mut self) -> Box<dyn Expression> {
        let ident = self.next_token().unwrap();
        if let Token::Identifier(id) = ident {
            if let Some(Token::Delimiter(Delimiter::LeftParen)) = self.peek_token() {
                debug!("Going to parse a call expression");
                self.next_token();
                let mut exprs = self.parse_expressions(Precedence::Lowest);
                exprs.reverse();
                debug!("Parsed a call expression");
                Box::new(CallExpression { name: id, exprs })
            } else {
                Box::new(IdentifierExpression { identifier: id })
            }
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

    fn parse_minus_unary_expression(&mut self) -> MinusUnaryExpression {
        self.next_token();
        MinusUnaryExpression {
            expr: self.parse_expression(Precedence::Prefix),
        }
    }

    fn parse_bang_unary_expression(&mut self) -> BangUnaryExpression {
        self.next_token();
        BangUnaryExpression {
            expr: self.parse_expression(Precedence::Prefix),
        }
    }

    fn expect_next(&mut self, expected_token: Token) {
        match self.next_token() {
            Some(token) if token == expected_token => {}
            _ => panic!("Expect a = after the let keyword"),
        }
    }

    /// If there's a semicolon, consume it. This method is useful at the end of
    /// parsing a statement, such as a let statement.
    fn consume_semicolon(&mut self) {
        if let Some(token) = self.peek_token() {
            if token == Token::Delimiter(Delimiter::Semicolon) {
                self.next_token();
            }
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

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_let_statements() {
        init();
        let data = vec![
            ("let x = 5;", "x", "5"),
            ("let x = 5 + 6", "x", "(5 + 6)"),
            ("let x = call(5, 6 + 7)", "x", "call(5, (6 + 7))"),
        ];
        for (input, identifier, expr) in data {
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);
            let stmt = parser.parse_let_statement();
            assert_eq!(stmt.name, String::from(identifier));
            assert_eq!(stmt.expr.to_string(), String::from(expr));
        }
    }

    #[test]
    fn test_return_statements() {
        init();
        let data = vec![
            ("return x;", "x"),
            ("return 5 + 6", "(5 + 6)"),
            ("return call(5, 6 + 7)", "call(5, (6 + 7))"),
        ];
        for (input, expr) in data {
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);
            let stmt = parser.parse_return_statement();
            assert_eq!(stmt.expr.to_string(), String::from(expr));
        }
    }

    #[test]
    fn test_expression_statements() {
        init();
        let data = vec![
            ("foobar;", "ExpressionStatement(foobar)"),
            ("5;", "ExpressionStatement(5)"),
            ("-50;", "ExpressionStatement((-50))"),
        ];
        for (input, expr) in data {
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);
            let stmt = parser.parse_expression_statement(Precedence::Lowest);
            debug!("test_expression_statements got: {}", stmt);
            assert_eq!(stmt.to_string(), String::from(expr));
        }
    }

    #[test]
    fn test_identifier_expression() {
        init();
        let input = " foobar; ";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let expr = parser.parse_identifier_expression();
        assert_eq!("foobar", expr.to_string());
    }

    #[test]
    fn test_call_expression() {
        init();
        let input = " foobar(a + b); ";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let expr = parser.parse_identifier_expression();
        assert_eq!("foobar((a + b))", expr.to_string());
    }

    #[test]
    fn test_literal_expression() {
        init();
        let input = " 50; ";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let expr = parser.parse_literal_expression();
        assert_eq!(LiteralIntegerExpression { literal: 50 }, expr);
    }

    #[test]
    fn test_bool_expression() {
        init();
        test_expression_helper(vec![("true;", "true"), ("false;", "false")]);
    }

    #[test]
    fn test_unary_expression() {
        init();
        test_expression_helper(vec![
            ("-50;", "(-50)"),
            ("!true;", "(!true)"),
            ("!false", "(!false)"),
        ]);
    }

    #[test]
    fn test_grouped_expression() {
        init();
        test_expression_helper(vec![
            ("(5 + 5) * 2", "((5 + 5) * 2)"),
            ("2 / (5 + 5)", "(2 / (5 + 5))"),
        ]);
    }

    #[test]
    fn test_parse_expression() {
        init();
        let pairs = vec![
            ("5 + 6 + 7;", "((5 + 6) + 7)"),
            ("5 - 6 + 7;", "((5 - 6) + 7)"),
            ("5 - 6 * 7;", "(5 - (6 * 7))"),
            ("5 - 6 * 7 + 3 / 4;", "((5 - (6 * 7)) + (3 / 4))"),
            ("a + b * c + d / e - f;", "(((a + (b * c)) + (d / e)) - f)"),
            (
                "3 + 4 * 5 == 3 * 1 + 4 * 5",
                "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
            ),
            ("-5 + 10 * 3", "((-5) + (10 * 3))"),
            ("!5 + 10 * 3", "((!5) + (10 * 3))"),
            ("-a * b", "((-a) * b)"),
            ("!-a", "(!(-a))"),
            ("a + b + c", "((a + b) + c)"),
            ("a + b - c", "((a + b) - c)"),
            ("a * b * c", "((a * b) * c)"),
            ("a * b / c", "((a * b) / c)"),
            ("a + b / c", "(a + (b / c))"),
            ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
            ("3 + 4;", "(3 + 4)"),
            ("-5 * 5", "((-5) * 5)"),
            ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
            (
                "3 + 4 * 5 == 3 * 1 + 4 * 5",
                "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
            ),
            ("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"),
            ("true", "true"),
            ("false", "false"),
            ("3 > 5 == false", "((3 > 5) == false)"),
            ("3 < 5 == true", "((3 < 5) == true)"),
            ("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)"),
            ("(5 + 5) * 2", "((5 + 5) * 2)"),
            ("2 / (5 + 5)", "(2 / (5 + 5))"),
            ("(5 + 5) * 2 * (5 + 5)", "(((5 + 5) * 2) * (5 + 5))"),
            ("-(5 + 5)", "(-(5 + 5))"),
            ("!(true == true)", "(!(true == true))"),
            ("add(b * c)", "add((b * c))"),
            ("a + add(b * c) + d", "((a + add((b * c))) + d)"),
            (
                "add(a + b + c * d / f + g)",
                "add((((a + b) + ((c * d) / f)) + g))",
            ),
            (
                "add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))",
                "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))",
            ),
        ];

        test_expression_helper(pairs);
    }

    /// This helper takes a vector of pairs of strings. It parses each input as an expression
    /// and checks the expression's string representation.
    fn test_expression_helper(pairs: Vec<(&str, &str)>) {
        for (input, repr) in pairs {
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);
            let expr = parser.parse_expression(Precedence::Lowest);
            assert_eq!(expr.to_string(), repr);
        }
    }
}
