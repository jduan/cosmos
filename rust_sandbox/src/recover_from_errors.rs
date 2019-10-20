// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

use std::fs::File;
use std::io::ErrorKind;

pub fn run() {
    // open_file();
    open_or_create_file();
    open_or_create_file2();
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
