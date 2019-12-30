pub mod ast;
pub mod lexer;
pub mod parser;
pub mod repl;

fn main() {
    println!("This is REPL of the Monkey Language.");
    repl::run();
}
