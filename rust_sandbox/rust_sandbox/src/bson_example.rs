use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: i32,
    phones: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use bson::Bson;

    #[test]
    fn bson_macro() {
        let s = Bson::String("Hello, world!".to_string());
        let s2 = bson!("Hello, world!".to_string());
        assert_eq!(s, s2);

        // You can have arrays of mixed rust raw types because all Bson types are
        // of the same "Bson" enum type.
        let a = Bson::Array(vec![Bson::Int32(5), Bson::Boolean(false)]);
        let a2 = bson!([5, false]);
        assert_eq!(a, a2);

        // You can access the underlying native Rust types.
        let f = Bson::Int32(123);
        assert_eq!(123, f.as_i32().unwrap());
    }

    #[test]
    fn structured_bson() {
        let bson_data = bson!({
            "name": "John Doe",
            "age": 43,
            "phones": [
                "12345",
                "23456",
            ]
        });
        let bson_data_copy = bson_data.clone();

        // Deserialize a Person struct from the bson data.
        match bson::from_bson::<Person>(bson_data) {
            Ok(p) => {
                println!("Person: {:?}", p);
                assert_eq!("John Doe".to_string(), p.name);

                // Serialize the Person object to bson data.
                let bson_data2 = bson::to_bson(&p).unwrap();
                assert_eq!(bson_data_copy, bson_data2);
            }
            Err(err) => println!("Failed to deserialize: {:?}", err),
        }
    }
}
