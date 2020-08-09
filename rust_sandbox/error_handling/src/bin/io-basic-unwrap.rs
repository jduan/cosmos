/// This program uses "unwrap" liberally. It allows you to focus on your program instead of
/// error handling, and it exposes the points where proper error handling needs to occur.
/// You can refactor it later to use better error handling.
use std::env;
use std::fs;
use std::path::Path;

/// The "io::Error" type in particular is pervasive throughout the standard library!

/// Read content of a file, parse it as a number, and double it.
fn file_double<P: AsRef<Path>>(file_path: P) -> i32 {
    let content = fs::read_to_string(file_path).unwrap();
    content.trim().parse::<i32>().unwrap() * 2
}

fn main() {
    let file_path = env::args().nth(1).unwrap();
    let doubled = file_double(file_path);
    println!("{}", doubled);
}
