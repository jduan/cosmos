// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

use std::fs;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

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
