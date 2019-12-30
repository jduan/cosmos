pub mod lexer;
pub mod repl;

fn main() {
    println!("This is REPL of the Monkey Language.");
    repl::run();
}
