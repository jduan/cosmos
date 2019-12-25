/// # There are 2 types of error handling in Rust: panic and Results.
///
/// Ordinary errors are handled using Results. These are typically caused by things outside the
/// program, like erroneous input, a network outage, or a permissions problem. That such
/// /situations occur is not up to us; even a bug-free program will encounter them from time to
/// time. Most of this chapter is dedicated to that kind of error.
///
/// Panic is for the other kind of error, the kind that "should never happen"! A program panics
/// /when it encounters something so messed up that there must be a bug in the program itself, such
/// as:
/// * out-of-bounds array access
/// * integer division by zero
/// * calling .unwrap() on an Optional that happens to be None
/// * assertion failures
/// (there's also the macro panic!(), for cases where your own code discovers that it has gone
/// wrong, and you therefore need to trigger a panic directly youself.)
/// What all these conditions have in common is that they are all the programmer's fault. A good
/// rule of thumb is: "Don't panic".
///
/// But we all make mistakes. When a panic happens, Rust does one of 2 things:
/// * unwind the stack (default behavior)
/// * abort the process
///
/// Panic is per thread. One thread can be panicking while other threads are going on about their
/// normal business.
///
/// If you compile with -C panic=abort, the first panic in your program immediately aborts the
/// process. (With this option, Rust does not need to know how to unwind the stack, so this can
/// reduce the size of your compiled code.)
///
/// There is not much to say about panics, because ordinary Rust code has no obligation to handle
/// panic. Even if you do use threads or catch_unwind(), all your panic-handling code will /likely
/// be concentrated in a few places. It’s unreasonable to expect every function in a program to
/// anticipate and cope with bugs in its own code.
///
/// Handle errors that are wrapped in Result.
///
/// The most thorough way of dealing with a Result is to use a match expression:
///
/// match get_weather(hometown) {
///      Ok(report) => {
///          display_weather(hometown, &report);
///      }
///      Err(err) => {
///          println!("error querying the weather: {}", err);
///          schedule_weather_retry();
///      }
/// }
///
/// There are other convenient methods for common cases:
///
/// 1. result.is_ok() and result.is_err() return a bool
/// 2. result.ok() returns the success type as an Option<T>. If result is a success result, this
///    returns Some(success_value); otherwise, it returns None, discarding the error value.
/// 3. result.err() returns the error value, if any, as an Option<E>
/// 4. result.unwrap_or(fallback) returns the success value, or it returns the "fallback" value,
///    discarding the error value.
/// 5. result.unwrap_or_else(fallback_fn) is the same but you pass a function or closure. This is
///    for cases where it would be wasteful to compute a fallback value while you're not going to
///    use it.
/// 6. result.unwrap() returns the success value or panics
/// 7. result.expect(message) is the same as .unwrap() but lets you provide a message when it /panics
///
/// Lastly, two methods for borrowing references to the value in a Result:
/// 1. result.as_ref() converts a Result<T, E> to a Result<&T, &E>
/// 2. result.as_mut() is the same but borrows a mutable reference, Result<&mut T, &mut E>
///
/// One reason these last 2 methods are useful is that all of the other methods listed above,
/// except .is_ok() and .is_err(), consume the result they operate on. For example, if you want to
/// call result.ok() but you need result to be left intact. You can write "result.as_ref.ok()",
/// which borrows result, returning an Option<&T> rather than an Option<T>
///
/// Propagating Errors
///
/// Rust has a ? operator to propagate errors up the call stack:
/// * On success, it unwraps the Result to get the success value inside.
/// * On error, it immediately returns from the enclosing function, passing the error result up /the
/// call chain. To ensure that this works, ? can only be used in functions that have a Result /return
/// type.
///
/// let weather = get_weather(hometown)?
///
/// is equivalent to:
///
/// let weather = match get_weather(hometown) {
///      Ok(success_value) => success_value,
///      Err(err) => return Err(err)
/// }
///
/// Working with Multiple Error Types
///
/// Often, more than one thing can go wrong and they may return different error types. There are a
/// few ways of dealing with this.
///
/// 1. Define a common Error type for your code and convert other errors like "io::Error" to your
///    own error type. The "error-chain" crate can help you with this.
///
/// 2. There's a standard error type in Rust: std::error::Error, which represents "any error". You
///    can use this error type for your own code too.
///
/// Dealing with Errors That "Can't Happen"
///
/// Sometimes we just know that an error can’t happen. For example, you parse a file of numbers.
///
///      let num = digits.parse::<u64>();
///
/// "num" is a result type. Since you know errors can't happen, you can use "unwrap" or "expect"/ to
/// get the unwrapped value (or panic if an error does happen):
///
///      let num = digits.parse::<u64>().unwrap();
///      let num = digits.parse::<u64>().expect("Expected a number in string format");
///
///  Ignore Errors
///
///  Occasionally we just want to ignore an error altogether. For example, in our print_error()
///  function, we had to handle the unlikely situation where printing the error triggers another
///  error. This could happen, for example, if stderr is piped to another process, and that /process
///  is killed.
///
///      writeln!(stderr(), "error: {}", err);  // warning: unused result
///      let _ = writeln!(stderr(), "error: {}", err);  // idiom: use "let _ =" to silence warning
///
///  Handling Errors in main()
///
///  In most places where a Result is produced, letting the error bubble up to the caller is the
///  right behavior. This is why ? is a single character in Rust. But if you propagate an error long
///  enough, eventually it reaches main(), and that’s where this approach has to stop. main() can’t
///  use ? because its return type is not Result. You can handle errors in main like this:
///
///      fn main() {
///          if let Err(err) = calculate_tides() {
///              print_error(&err);
///              std::process::exit(1);
///          }
///      }
///
///
/// # Why Results?
///
/// Rust requires the programmer to make some sort of decision, and record it in
/// the code, at every point where an error could occur. This is good because
/// otherwise, it’s easy to get error handling wrong through neglect.
///
/// The most common decision is to allow errors to propagate, and that’s written
/// with a single character, ‘?’. Thus error plumbing does not clutter up your code
/// the way it does in C and Go. Yet it’s still visible: you can look at a chunk of
/// code and see at a glance all places where errors are propagated.
///
/// Since the possibility of errors is part of every function’s return type,
/// it’s clear which functions can fail and which can’t. If you change a function
/// to be fallible, you’re changing its return type, so the compiler will make you
/// update that function’s downstream users.
///
/// Rust checks that Result values are used, so you can’t accidentally let an
/// error pass silently (a common mistake in C).
///
/// Since Result is a data type like any other, it’s easy to store success and
/// error results in the same collection. This makes it easy to model partial
/// success. For example, if you’re writing a program that loads millions of
/// records from a text file, and you need a way to cope with the likely outcome
/// that most will succeed, but some will fail, you can represent that situation in
/// memory using a vector of Results.
///
/// The cost is that you’ll find yourself thinking about and engineering error
/// handling more in Rust than you would in other languages. As in many other
/// areas, Rust’s take on error handling is wound just a little tighter than what
/// you’re used to. For systems programming, it’s worth it.
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;
use std::io::{stderr, Write};

pub fn run() {
    // open_file();
    open_or_create_file();
    open_or_create_file2();
    // unwrap_or_expect();
    match propagate_errors() {
        Ok(username) => println!("username is {}", username),
        Err(e) => println!("failed to read username from file {:?}", e),
    }
    the_question_operator();
    the_question_operator2();
    the_question_operator3();
}

fn open_file() {
    let f = File::open("/tmp/fjdksljfdfd");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}

fn open_or_create_file() {
    let s = "/tmp/hello.txt";
    let f = File::open(s);

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(s) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
}

// This version is better than the one above.
fn open_or_create_file2() {
    let s = "/tmp/hello2.txt";

    let f = File::open(s).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(s).unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

fn unwrap_or_expect() {
    let f = File::open("/tmp/hello.txt").unwrap();
    let f = File::open("/tmp/world.txt").expect("Failed to open file /tmp/world.txt");
}

// Read the username from the file. Return an error if something bad happens.
fn propagate_errors() -> Result<String, io::Error> {
    let f = File::open("/tmp/hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// Use the ? operator to simplify the code above!
// ? works on the "Result" type.
// How does "?" work:
// 1. If the value of the Result is an Ok, the value inside the Ok will get returned from the
//    expression, and the program will continue.
// 2. If the value is an Err, the Err will be returned from the whole function as if we had used
//    the "return" keyword so the error gets propagated to the calling code.
fn the_question_operator() -> Result<String, io::Error> {
    let mut f = File::open("/tmp/hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// chaining
fn the_question_operator2() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("/tmp/hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// the shortest
// "fs::read_to_string" is a function that opens the file, creates a new String, reads the contents
// of the file, puts the contents into that String, and returns it.
fn the_question_operator3() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

/// Define your own error type and have it work like the standard error types.

#[derive(Debug, Clone)]
pub struct JsonError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

use std;
use std::fmt;
use std::num::ParseIntError;

// Errors should be printable.
impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{} ({}:{})", self.message, self.line, self.column)
    }
}

// Errors should implement the std::error::Error trait.
impl std::error::Error for JsonError {
    fn description(&self) -> &str {
        &self.message
    }
}

/// When you want to reuse the same specific Result type many times in a module,
/// you can define an alias. At a module level, creating aliases can be particularly
/// helpful. Errors found in a specific module often have the same Err type, so a
/// single alias can succinctly define all associated Results. This is so useful
/// that the std library even supplies one: io::Result!
type AliasedResult<T> = Result<T, ParseIntError>;

/// The Result type has various combinators: map, and_then.
fn multiply(first_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str
            .parse::<i32>()
            .map(|second_number| first_number * second_number)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply_two_numbers() {
        assert_eq!(200, multiply("10", "20").unwrap());
        assert!(multiply("not a number", "20").is_err());
    }
}
