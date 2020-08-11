/// This program map all errors into their string representation. It uses "early returns" to
/// make the code a bit easier to follow.
use std::env;
use std::fs;
use std::path::Path;

/// The "io::Error" type in particular is pervasive throughout the standard library!

/// Read content of a file, parse it as a number, and double it.
fn file_double<P: AsRef<Path>>(file_path: P) -> Result<i32, String> {
    let content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(err) => return Err(err.to_string()),
    };

    let n = match content.trim().parse::<i32>() {
        Ok(n) => n,
        Err(err) => return Err(err.to_string()),
    };

    Ok(2 * n)
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
