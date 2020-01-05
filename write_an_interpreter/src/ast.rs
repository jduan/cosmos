use log::*;

use crate::lexer::Operator;
use std::fmt::{Display, Error, Formatter};

pub trait Node: Display {}

pub trait Statement: Node {}

pub trait Expression: Node {}

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Display for Program {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        writeln!(f, "Program has the following statements:")?;
        for stmt in &self.statements {
            writeln!(f, "> {}", stmt)?;
        }

        Ok(())
    }
}

impl Program {
    pub fn new() -> Self {
        Self { statements: vec![] }
    }

    pub fn add_statement(&mut self, statement: Box<dyn Statement>) {
        debug!("Parsed one statement: {}", statement);
        self.statements.push(statement);
    }
}

pub struct LetStatement {
    pub name: String,
    pub expr: Box<dyn Expression>,
}

impl Node for LetStatement {}
impl Statement for LetStatement {}
impl Display for LetStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "LetStatement({} = {})", self.name, self.expr)
    }
}

pub struct ReturnStatement {
    pub expr: Box<dyn Expression>,
}

impl Node for ReturnStatement {}
impl Statement for ReturnStatement {}
impl Display for ReturnStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "ReturnStatement({})", self.expr)
    }
}

pub struct ExpressionStatement {
    pub expr: Box<dyn Expression>,
}

impl Node for ExpressionStatement {}
impl Statement for ExpressionStatement {}
impl Display for ExpressionStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "ExpressionStatement({})", self.expr)
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct IdentifierExpression {
    pub identifier: String,
}

impl Node for IdentifierExpression {}
impl Expression for IdentifierExpression {}

impl Display for IdentifierExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.identifier)
    }
}

pub struct CallExpression {
    pub name: String,
    pub exprs: Vec<Box<dyn Expression>>,
}

impl Node for CallExpression {}
impl Expression for CallExpression {}

impl Display for CallExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}(", self.name)?;
        let len = self.exprs.len();
        for i in 0..len {
            write!(f, "{}", self.exprs[i])?;
            if i != len - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, ")")
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct BoolLiteralExpression {
    pub literal: bool,
}

impl Node for BoolLiteralExpression {}
impl Expression for BoolLiteralExpression {}

impl Display for BoolLiteralExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.literal)
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct LiteralIntegerExpression {
    pub literal: i32,
}

impl Node for LiteralIntegerExpression {}
impl Expression for LiteralIntegerExpression {}

impl Display for LiteralIntegerExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.literal)
    }
}

pub struct MinusUnaryExpression {
    pub expr: Box<dyn Expression>,
}

impl Node for MinusUnaryExpression {}
impl Expression for MinusUnaryExpression {}

impl Display for MinusUnaryExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "(-{})", self.expr)
    }
}

pub struct BangUnaryExpression {
    pub expr: Box<dyn Expression>,
}

impl Node for BangUnaryExpression {}
impl Expression for BangUnaryExpression {}

impl Display for BangUnaryExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "(!{})", self.expr)
    }
}

pub struct PrefixExpression {
    pub operator: Operator,
    pub expr: Box<dyn Expression>,
}

impl Node for PrefixExpression {}
impl Expression for PrefixExpression {}

impl Display for PrefixExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "PrefixExpression (operator: {:?}, expr: {})",
            self.operator, self.expr
        )
    }
}

pub struct InfixExpression {
    pub left_expr: Box<dyn Expression>,
    pub operator: Operator,
    pub right_expr: Box<dyn Expression>,
}

impl Node for InfixExpression {}
impl Expression for InfixExpression {}

impl Display for InfixExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "({} {} {})",
            self.left_expr, self.operator, self.right_expr
        )
    }
}
