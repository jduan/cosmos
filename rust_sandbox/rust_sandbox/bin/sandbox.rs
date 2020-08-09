#[macro_use]
extern crate bson;

use bson::Bson;
use serde::{Deserialize, Serialize};

pub fn main() {
    println!("Hello World from Rust!");
    typed_example();
}

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: i32,
    phones: Vec<String>,
}

fn typed_example() {
    // Some BSON input data as a `Bson`.
    let bson_data: Bson = bson!({
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });
    let bson_data_copy = bson_data.clone();

    // Deserialize the Person struct from the BSON data, automatically
    // verifying that the necessary keys are present and that they are of
    // the correct types.
    let mut person: Person = bson::from_bson(bson_data).unwrap();

    // Do things just like with any other Rust data structure.
    println!("Redacting {}'s record.", person.name);
    person.name = "REDACTED".to_string();

    // Get a serialized version of the input data as a `Bson`.
    let redacted_bson = bson::to_bson(&person).unwrap();

    // They are not equal because name has been changed.
    assert_ne!(redacted_bson, bson_data_copy);
}
