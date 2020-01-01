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

#[derive(PartialEq, Eq, Debug)]
pub struct LetStatement {
    pub name: String,
    //    value: Box<dyn Expression>,
}

impl Node for LetStatement {}
impl Statement for LetStatement {}
impl Display for LetStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "LetStatement({} = expr)", self.name)?;
        Ok(())
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct ReturnStatement {
    //    expr: Box<dyn Expression>,
}

impl Node for ReturnStatement {}
impl Statement for ReturnStatement {}
impl Display for ReturnStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "ReturnStatement")?;
        Ok(())
    }
}

pub struct ExpressionStatement {
    pub expr: Box<dyn Expression>,
}

impl Node for ExpressionStatement {}
impl Statement for ExpressionStatement {}
impl Display for ExpressionStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "ExpressionStatement (expr = {})", self.expr)?;
        Ok(())
    }
}

/// Identifers are the simplest expression. They evaluate to they value they are bound to.
#[derive(PartialEq, Eq, Debug)]
pub struct IdentifierExpression {
    pub identifier: String,
}

impl Node for IdentifierExpression {}
impl Expression for IdentifierExpression {}

impl Display for IdentifierExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "IdentifierExpression (id = {})", self.identifier)?;
        Ok(())
    }
}

/// Identifers are the simplest expression. They evaluate to they value they are bound to.
#[derive(PartialEq, Eq, Debug)]
pub struct LiteralIntegerExpression {
    pub literal: i32,
}

impl Node for LiteralIntegerExpression {}
impl Expression for LiteralIntegerExpression {}

impl Display for LiteralIntegerExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "LiteralExpression ({})", self.literal)?;
        Ok(())
    }
}

/// Identifers are the simplest expression. They evaluate to they value they are bound to.
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
        )?;
        Ok(())
    }
}
