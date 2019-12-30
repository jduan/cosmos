use crate::lexer::SpecialToken;
use crate::lexer::{Lexer, Token};
use std::io;
use std::io::Write;

static PROMPT: &'static str = "> ";

pub fn run() {
    loop {
        let mut line = String::new();
        print!("{}", PROMPT);
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        if line.is_empty() {
            break;
        }
        let mut lexer = Lexer::new(&line);
        loop {
            let token = lexer.next_token();
            if token == Token::SpecialToken(SpecialToken::EOF) {
                break;
            }
            println!("{:?}", token);
        }
    }
}
