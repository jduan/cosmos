use crate::lexer::Lexer;

trait Node {}

trait Statement: Node {}

trait Expression: Node {}

pub struct Program {
    statements: Vec<Box<dyn Statement>>,
}

pub struct LetStatement {
    name: Identifier,
    value: Expression,
}

struct Identifier {
    name: String,
}

impl Node for Identifier {}
impl Expression for Identifier {}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Parser<'a> {
        Parser { lexer }
    }

    //    pub fn parse_program() -> Program {}
}
