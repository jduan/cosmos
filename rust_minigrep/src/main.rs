use std::env;
use std::process;

use rust_minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // similar to unwrap_or_else but since we don't care about the return value (which would be ())
    // in the Ok case, we use "let Err(e)" to handle the error case only.
    if let Err(e) = rust_minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
