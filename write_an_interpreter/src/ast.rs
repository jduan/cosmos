pub trait Node {}

pub trait Statement: Node {}

pub trait Expression: Node {}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
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

pub type Identifier = String;

impl Node for Identifier {}
impl Expression for Identifier {}
