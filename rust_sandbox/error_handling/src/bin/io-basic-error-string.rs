/// This program map all errors into their string representation.
use std::env;
use std::fs;
use std::path::Path;

/// The "io::Error" type in particular is pervasive throughout the standard library!

/// Read content of a file, parse it as a number, and double it.
fn file_double<P: AsRef<Path>>(file_path: P) -> Result<i32, String> {
    fs::read_to_string(file_path)
        .map_err(|err| err.to_string())
        .and_then(|content| content.trim().parse::<i32>().map_err(|err| err.to_string()))
        .map(|n| 2 * n)
}

fn main() {
    match env::args().nth(1) {
        Some(file_path) => match file_double(file_path) {
            Ok(n) => println!("{}", n),
            Err(err) => println!("Error: {}", err),
        },
        None => println!("Expect one argument"),
    }
}
