use std::io;
use std::num;
use std::path::Path;
use std::{env, fs};

/// This is the ideal error handling!
/// * You define your own, custom error types that callers of your code can look at.
/// * Your custom error types implement the Error trait (hence also Debug and Display)
/// * You implement the From traits for other Errors that be converted to your own error types.
/// * The ? operator is used to implicitly convert other error types into your error types.
///
/// You could also implement the Error trait. See [standard-error-type.rs]

#[derive(Debug)]
enum CliError {
    Io(io::Error),
    Parse(num::ParseIntError),
}

impl From<io::Error> for CliError {
    fn from(err: io::Error) -> CliError {
        CliError::Io(err)
    }
}

impl From<num::ParseIntError> for CliError {
    fn from(err: num::ParseIntError) -> CliError {
        CliError::Parse(err)
    }
}

/// Read content of a file, parse it as a number, and double it.
fn file_double<P: AsRef<Path>>(file_path: P) -> Result<i32, CliError> {
    // Note that we don't need to call "map_err" here because we already implemented the From
    // trait for the Errors that can be thrown by these calls.
    let content = fs::read_to_string(file_path)?;
    let n = content.trim().parse::<i32>()?;

    Ok(2 * n)
}

fn main() {
    match env::args().nth(1) {
        Some(file_path) => match file_double(file_path) {
            Ok(n) => println!("{}", n),
            Err(err) => println!("Error: {:?}", err),
        },
        None => println!("Expect one argument"),
    }
}
