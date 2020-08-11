use std::io;
use std::num;
use std::path::Path;
/// Using String for your errors has some downsides:
///
/// 1. The error messages tend to clutter your code. It’s possible to define the error messages
/// elsewhere, but unless you’re unusually disciplined, it is very tempting to embed the error
/// message into your code.
///
/// 2. The second and more important downside is that Strings are lossy. That is, if all errors
/// are converted to strings, then the errors we pass to the caller become completely opaque.
/// The only reasonable thing the caller can do with a String error is show it to the user.
///
/// In contrast, the io::Error type embeds an io::ErrorKind, which is "structured data" that
/// represents what went wrong during an IO operation.
///
/// A rule of thumb is to define your own error type, but a String error type will do in a pinch,
/// particularly if you’re writing an application. If you’re writing a library, defining your own
/// error type should be strongly preferred so that you don’t remove choices from the caller
/// unnecessarily.
use std::{env, fs};

#[derive(Debug)]
enum CliError {
    Io(io::Error),
    Parse(num::ParseIntError),
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
