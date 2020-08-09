use std::error::Error;
use std::path::Path;
use std::{env, fs};
/// The "std::convert::From" trait.
///
/// trait From<T> {
///     fn from(T) -> Self;
/// }
///
/// The From trait provides a generic way to talk about conversion from a particular type T to
/// some other type (ie: the subject of the impl, or Self). The crux of From is the set of
/// implementations provided by the standard library!
///
/// There's one critical impl:
///
/// impl<'a, E: Error + 'a> From<E> for Box<Error + 'a>
///
/// It says that for any type that impls Error, we can convert it to a "trait object" Box<Error>.
///
/// Time to revisit the ? operator. It's defined as:
///
/// match ::std::ops::Try::into_result(x) {
///     Ok(v) => v,
///     Err(e) => return ::std::ops::Try::from_error(From::from(e)),
/// }

/// We are getting very close to ideal error handling. Our code has very little overhead as a
/// result from error handling because the ? operator encapsulates three things simultaneously:
///
///  1. Case analysis.
///  2. Control flow.
///  3. Error type conversion.
///
/// There’s one little nit left: the Box<Error> type is opaque. If we return a Box<Error> to the
/// caller, the caller can’t (easily) inspect underlying error type.
fn file_double<P: AsRef<Path>>(file_path: P) -> Result<i32, Box<dyn Error>> {
    // Note that here we don't need to call "map_err" because the From trait has an impl that
    // let's it convert any error type into a Box<Error>
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
