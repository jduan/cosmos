use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::io;
use std::num;
use std::path::Path;
use std::{env, fs};
/// The standard library defines an Error trait.
///
/// Errors must describe themselves through the Display and Debug traits, and may provide cause
/// chain info via the "source" method.

#[derive(Debug)]
enum CliError {
    Io(io::Error),
    Parse(num::ParseIntError),
}

impl Display for CliError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            // Both underlying errors always impl Display, so we defer to their implementations.
            CliError::Io(err) => write!(f, "IO error: {}", err),
            CliError::Parse(err) => write!(f, "Parse error: {}", err),
        }
    }
}

impl Error for CliError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        // Both of these implicitly cast "err" from their concrete types to a "trait object":
        // &Error. This works because both error types implement Error.
        match self {
            CliError::Io(err) => Some(err),
            CliError::Parse(err) => Some(err),
        }
    }
}

/// Read content of a file, parse it as a number, and double it.
fn file_double<P: AsRef<Path>>(file_path: P) -> Result<i32, CliError> {
    let content = fs::read_to_string(file_path).map_err(CliError::Io)?;
    let n = content.trim().parse::<i32>().map_err(CliError::Parse)?;

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
