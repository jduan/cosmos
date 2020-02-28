/// Summary
/// 1. If you're writing quick 'n' dirty code that would be overburdened by error handling, it's
/// probably just fine to use unwrap or expect.
/// 2. If you're writing a quick 'n' dirty program and feel ashamed about panicking anyway, then
/// using either a Result of String or a Box<Error + Send + Sync> for your error type.
/// 3. Otherwise, in a program, define your own error types with appropriate From and Error impls
/// so your error types work with the ? operator.
/// 4. If you're writing a library and your code can produce errors, define your own error type
/// and implement the std::error::Error trait. Where appropriate, implement From to make both
/// your library code and the caller's code easier to write. (Because of Rust's coherence rules,
/// callers will not be able to impl From on your error type, so your library should do it.)
/// 5. Learn the combinators defined on Option and Result, such as map, unwrap_or, and_then, etc.
use serde::export::Formatter;
use serde::Deserialize;
use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::io::Read;
use std::num::ParseIntError;
use std::path::Path;

/// https://doc.rust-lang.org/1.5.0/book/error-handling.html

/// The Option type

// Searches `haystack` for the Unicode character `needle`. If one is found, the
// byte offset of the character is returned. Otherwise, `None` is returned.
pub fn find(haystack: &str, needle: char) -> Option<usize> {
    for (offset, ch) in haystack.char_indices() {
        if ch == needle {
            return Some(offset);
        }
    }

    None
}

// Returns the extension of the given file name.
// If `file_name` has no `.`, then `None` is returned.
pub fn get_extension(filename: &str) -> Option<&str> {
    match find(filename, '.') {
        None => None,
        Some(offset) => Some(&filename[offset + 1..]),
    }
}

// Use the "map" combinator. This is Functor in Haskell's term.
pub fn get_extension2(filename: &str) -> Option<&str> {
    find(filename, '.').map(|offset| &filename[offset + 1..])
}

// "and_then" is a Monad!
// map vs and_then
// 1. map: takes a function that does something only with the inner value. The result of
// that function is then always rewrapped with Some.
// 2. and_then: takes a function that returns another Option on its own. It needs no rewrapping.
pub fn file_path_ext_explicit(file_path: &str) -> Option<&str> {
    file_name(file_path).and_then(|name| get_extension(name))
}

// This is a fake implementation.
pub fn file_name(file_path: &str) -> Option<&str> {
    if file_path == "hello_world.rs" {
        Some(file_path)
    } else {
        None
    }
}

/// The Result type
///
/// Instead of expressing the possibility of absence like Option does, Result expresses
/// the possibility of error. Usually, the error is used to explain why the result of
/// some computation failed. This is a strictly more general form of Option. Consider the
/// following type alias, which is semantically equivalent to the real Option<T> in every way:
///
/// type Option<T> = Result<T, ()>;
///
/// Like Option, the Result type also has methods like "unwrap", and combinators like
/// "map", "and_then", etc.

/// The Result type alias idiom
/// Why would we do this? Well, if we have a lot of functions that could return ParseIntError,
/// then it's much more convenient to define an alias that always uses ParseIntError so that
/// we don't have to write it out all the time.
type MyResult<T> = std::result::Result<T, ParseIntError>;

pub fn double_number(number_str: &str) -> MyResult<i32> {
    let number = number_str.parse::<i32>()?;
    Ok(number * 2)
}

pub fn double_number2(number_str: &str) -> MyResult<i32> {
    number_str.parse::<i32>().map(|i| i * 2)
}

/// Working with multiple error types
///
/// 1. Compose Option and Result

pub fn double_first_arg(args: Vec<&str>) -> Result<i32, String> {
    match args.first() {
        None => Err(String::from("Need at least one argument")),
        Some(str) => str
            .parse::<i32>()
            .map(|i| i * 2)
            .map_err(|err| format!("Failed to parse '{}' into an int. Error: {:?}", str, err)),
    }
}

pub fn double_first_arg2(args: Vec<&str>) -> Result<i32, String> {
    args.first()
        // ok_or turns an Option into a Result
        .ok_or(String::from("Need at least one argument"))
        // We need "and_then" because "str.parse" returns another Result
        .and_then(|str| {
            str.parse::<i32>()
                .map(|i| i * 2)
                // We need "map_err" because the return type is a Result<i32, String>,
                // not a Result<i32, ParseIntError>
                .map_err(|err| format!("Failed to parse '{}' into an int. Error: {:?}", str, err))
        })
}

// Read a file (which contains a number) and doubles it.
pub fn file_double<P: AsRef<Path>>(file_path: P) -> i32 {
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let n: i32 = contents.trim().parse().unwrap();
    n * 2
}

// String is chosen as the common "error" type for all the possible errors that
// can occur in this function.
pub fn file_double2<P: AsRef<Path>>(file_path: P) -> Result<i32, String> {
    let mut file =
        File::open(file_path).map_err(|err| format!("Failed to open file: {:?}", err.kind()))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|err| format!("Failed to read file: {:?}", err))?;
    let n: i32 = contents
        .trim()
        .parse()
        .map_err(|err: ParseIntError| format!("Failed to parse file into an int: {:?}", err))?;
    Ok(n * 2)
}

// Same as file_double2 but uses combinators.
// and_then is the key!
// map_err is the trick that allows us to convert all kinds of errors into String.
//
// comparison: file_double2 uses "early return" style and is argubly more readable.
pub fn file_double3<P: AsRef<Path>>(file_path: P) -> Result<i32, String> {
    File::open(file_path)
        .map_err(|err| format!("Failed to open file: {:?}", err.kind()))
        .and_then(|mut file| {
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .map_err(|err| format!("Failed to read file: {:?}", err))
                .map(|_| contents)
        })
        .and_then(|contents| {
            contents
                .trim()
                .parse::<i32>()
                .map_err(|err: ParseIntError| {
                    format!("Failed to parse file into an int: {:?}", err)
                })
        })
        .map(|n| n * 2)
}

#[derive(Debug)]
pub enum FileDoubleError {
    Io(std::io::Error),
    Parse(ParseIntError),
}

/// A rule of thumb is to define your own error type, but a String error type will do
/// in a pinch, particularly if you're writing an application. If you're writing a library,
/// defining your own error type should be strongly preferred so that you don't remove
/// choices from the caller unnecessarily.
pub fn file_double4<P: AsRef<Path>>(file_path: P) -> Result<i32, FileDoubleError> {
    let mut file = File::open(file_path).map_err(FileDoubleError::Io)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(FileDoubleError::Io)?;
    let n: i32 = contents.trim().parse().map_err(FileDoubleError::Parse)?;
    Ok(n * 2)
}

/// Make FileDoubleError implement the Error trait!

impl Display for FileDoubleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            FileDoubleError::Io(err) => write!(f, "IO error: {}", err),
            FileDoubleError::Parse(err) => write!(f, "Parse error: {}", err),
        }
    }
}

impl std::error::Error for FileDoubleError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            FileDoubleError::Io(err) => Some(err),
            FileDoubleError::Parse(err) => Some(err),
        }
    }
}

/// This function compiles because of "std::convert::From". From is very useful because it
/// gives us a generic way to talk about conversion from a particular type T to some other type.
///
/// impl<'a, E: Error + 'a> From<E> for Box<Error + 'a>
///
/// This impl says that for any type that impls Error, we can convert it to a trait object
/// Box<Error>.
///
/// The downside is that since Box<Error> is a trait object, it erases the type, which means
/// the compiler can no longer reason about its underlying type.
pub fn file_double5<P: AsRef<Path>>(file_path: P) -> Result<i32, Box<dyn Error>> {
    // The ? actually calls: std::convert::From::from(err)
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let n: i32 = contents.trim().parse()?;
    Ok(n * 2)
}

impl From<std::io::Error> for FileDoubleError {
    fn from(err: std::io::Error) -> FileDoubleError {
        FileDoubleError::Io(err)
    }
}

impl From<ParseIntError> for FileDoubleError {
    fn from(err: ParseIntError) -> FileDoubleError {
        FileDoubleError::Parse(err)
    }
}

/// This function compiles because we have implemented the From trait for FileDoubleError.
/// The difference from "file_double4" is that we have got rid of those "map_err" translations.
pub fn file_double6<P: AsRef<Path>>(file_path: P) -> Result<i32, FileDoubleError> {
    // The ? actually calls: std::convert::From::from(err)
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let n: i32 = contents.trim().parse()?;
    Ok(n * 2)
}

/// Case study: A program to read population data
/// https://doc.rust-lang.org/1.5.0/book/error-handling.html#case-study-a-program-to-read-population-data

#[derive(Debug, Deserialize)]
struct Row {
    country: String,
    city: String,
    agency: String,
    region: String,
    population: Option<u64>,
    latitude: Option<f64>,
    longitude: Option<f64>,
}

/// This function returns a Box<dyn Error>
pub fn search_city<P: AsRef<Path>>(filepath: P, city: &str) -> Result<String, Box<dyn Error>> {
    let file = File::open(filepath)?;
    let mut rdr = csv::Reader::from_reader(file);
    for row in rdr.deserialize() {
        let record: Row = row?;
        if &record.city == city {
            return Ok(format!(
                "{} {}: {:?}",
                record.city,
                record.country,
                record.population.expect("population count")
            ));
        }
    }

    // This is how to convert a String to a Box<dyn Error>
    Err(Box::<dyn Error>::from(String::from("City not found")))

    // This also works because "From::from" is overloaded on both its argument and
    // its return type!
    // Err(From::from(String::from("City not found")))
}

#[derive(Debug)]
pub enum CityFinderError {
    Io(std::io::Error),
    Csv(csv::Error),
    NotFound,
}

impl Display for CityFinderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            CityFinderError::Io(err) => write!(f, "IO error: {}", err),
            CityFinderError::Csv(err) => write!(f, "Csv error: {}", err),
            CityFinderError::NotFound => write!(f, "City not found"),
        }
    }
}

impl std::error::Error for CityFinderError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CityFinderError::Io(err) => Some(err),
            CityFinderError::Csv(err) => Some(err),
            CityFinderError::NotFound => None,
        }
    }
}

impl From<std::io::Error> for CityFinderError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<csv::Error> for CityFinderError {
    fn from(err: csv::Error) -> Self {
        Self::Csv(err)
    }
}

/// This function returns a custom error type.
pub fn search_city2<P: AsRef<Path>>(filepath: P, city: &str) -> Result<String, CityFinderError> {
    let file = File::open(filepath)?;
    let mut rdr = csv::Reader::from_reader(file);
    for row in rdr.deserialize() {
        let record: Row = row?;
        if &record.city == city {
            return Ok(format!(
                "{} {}: {:?}",
                record.city,
                record.country,
                record.population.expect("population count")
            ));
        }
    }

    Err(CityFinderError::NotFound)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_find() {
        assert_eq!(Some(12), find("Hello, world!", '!'));
        assert_eq!(None, find("Hello, world!", '?'));

        // Use match pattern
        let filename = "foobar.rs";
        match find(filename, '.') {
            None => println!("No file extension found."),
            Some(i) => println!("Extension: {}", &filename[i + 1..]),
        }

        // Use unwrap
        // You can think of this style of error handling as similar to a bull running
        // through a china shop. The bull will get to where it wants to go, but it will
        // trample everything in the process.
        let offset = find(filename, '.').unwrap();
        assert_eq!(6, offset);
    }

    #[test]
    fn test_get_extension() {
        assert_eq!(None, get_extension("hello"));
        assert_eq!(Some("rs"), get_extension("hello.rs"));
    }

    #[test]
    fn test_get_extension1() {
        assert_eq!(None, get_extension2("hello"));
        assert_eq!(Some("rs"), get_extension2("hello.rs"));
        // use "unwrap_or"
        assert_eq!("rs", get_extension2("hello").unwrap_or("rs"));
    }

    #[test]
    fn test_file_path_ext_explicit() {
        assert_eq!(Some("rs"), file_path_ext_explicit("hello_world.rs"));
        assert_eq!(None, file_path_ext_explicit("the_book.rs"));
    }

    #[test]
    fn test_double_number() {
        assert_eq!(42, double_number("21").unwrap());
        assert!(double_number("hello").is_err());
    }

    #[test]
    fn test_double_number2() {
        assert_eq!(42, double_number2("21").unwrap());
        assert!(double_number2("hello").is_err());
    }

    #[test]
    fn test_double_first_arg() {
        assert_eq!(
            Err(String::from("Need at least one argument")),
            double_first_arg(vec![])
        );
        assert_eq!(
            Err(String::from(
                "Failed to parse 'hello' into an int. Error: ParseIntError { kind: InvalidDigit }"
            )),
            double_first_arg(vec!["hello"])
        );
    }

    #[test]
    fn test_double_first_arg2() {
        assert_eq!(
            Err(String::from("Need at least one argument")),
            double_first_arg2(vec![])
        );
        assert_eq!(
            Err(String::from(
                "Failed to parse 'hello' into an int. Error: ParseIntError { kind: InvalidDigit }"
            )),
            double_first_arg2(vec!["hello"])
        );
    }

    #[test]
    fn test_file_double() {
        let file = create_file("5").unwrap();
        assert_eq!(10, file_double(file.path()));
    }

    #[test]
    fn test_file_double2() {
        let file = create_file("5").unwrap();
        assert_eq!(Ok(10), file_double2(file.path()));
        assert_eq!(
            Err(String::from("Failed to open file: NotFound")),
            file_double2("/does/not/exist")
        );
        let file = create_file("hello").unwrap();
        assert_eq!(
            Err(String::from(
                "Failed to parse file into an int: ParseIntError { kind: InvalidDigit }"
            )),
            file_double2(file.path())
        );
    }

    #[test]
    fn test_file_double3() {
        let file = create_file("5").unwrap();
        assert_eq!(Ok(10), file_double3(file.path()));
        assert_eq!(
            Err(String::from("Failed to open file: NotFound")),
            file_double3("/does/not/exist")
        );
        let file = create_file("hello").unwrap();
        assert_eq!(
            Err(String::from(
                "Failed to parse file into an int: ParseIntError { kind: InvalidDigit }"
            )),
            file_double3(file.path())
        );
    }

    #[test]
    fn test_file_double4() {
        let file = create_file("5").unwrap();
        assert_eq!(10, file_double4(file.path()).unwrap());
        if let Err(FileDoubleError::Io(_)) = file_double4("/does/not/exist") {
            // expected
        } else {
            panic!("Expected a FileDoubleError::Io but didn't get one");
        }
        let file = create_file("hello").unwrap();
        if let Err(FileDoubleError::Parse(_)) = file_double4(file.path()) {
            // expected
        } else {
            panic!("Expected a FileDoubleError::Parse but didn't get one");
        }
    }

    #[test]
    fn test_file_double6() {
        let file = create_file("5").unwrap();
        assert_eq!(10, file_double6(file.path()).unwrap());
        if let Err(FileDoubleError::Io(_)) = file_double6("/does/not/exist") {
            // expected
        } else {
            panic!("Expected a FileDoubleError::Io but didn't get one");
        }
        let file = create_file("hello").unwrap();
        if let Err(FileDoubleError::Parse(_)) = file_double6(file.path()) {
            // expected
        } else {
            panic!("Expected a FileDoubleError::Parse but didn't get one");
        }
    }

    fn create_file(text: &str) -> std::result::Result<NamedTempFile, Box<dyn std::error::Error>> {
        let mut file = NamedTempFile::new()?;
        writeln!(file, "{}", text)?;

        Ok(file)
    }
}
