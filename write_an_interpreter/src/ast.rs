use std::fmt::{Display, Error, Formatter};

pub trait Node {}

pub trait Statement: Node + Display {}

pub trait Expression: Node + Display {}

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
        self.statements.push(statement);
    }
}

pub struct LetStatement {
    pub name: Identifier,
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

pub type Identifier = String;

impl Node for Identifier {}
impl Expression for Identifier {}
