use serde::{Deserialize, Serialize};

/// Parsing JSON as strongly typed data structures
#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::io::SeekFrom;
    use serde_json::{Result, Value};
    use std::io::Seek;

    /// Any valid JSON data can be manipulated in the following recursive enum representation.
    /// This data structure is serde_json::Value.
    ///
    /// enum Value {
    ///     Null,
    ///     Bool(bool),
    ///     Number(Number),
    ///     String(String),
    ///     Array(Vec<Value>),
    ///     Object(Map<String, Value>),
    /// }
    #[test]
    fn untyped_example() -> Result<()> {
        let data = get_json_data();

        // Parse the data into serde_json::Value
        let value: Value = serde_json::from_str(data)?;

        assert_eq!("John Doe", value["name"]);
        assert_eq!(43, value["age"]);
        assert_eq!("+44 1234567", value["phones"][0]);
        assert_eq!("+44 2345678", value["phones"][1]);

        // If the key doesn't exist, you get back Value::Null
        assert_eq!(Value::Null, value["bad key"]);

        Ok(())
    }

    #[test]
    fn typed_example() -> Result<()> {
        let data = get_json_data();

        // Parse data into structured data, a Person object.
        let person: Person = serde_json::from_str(data)?;

        assert_eq!("John Doe", person.name);
        assert_eq!(43, person.age);

        Ok(())
    }

    #[test]
    #[should_panic]
    fn bad_json_data() {
        let bad_data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
        }"#;

        serde_json::from_str::<Person>(bad_data).unwrap();
    }

    #[test]
    fn serialize_to_json() -> Result<()> {
        let person = Person {
            name: "John Wick".to_string(),
            age: 99,
            phones: vec!["415 200 3475".to_string(), "925 200 3475".to_string()],
        };

        let json_string = serde_json::to_string(&person)?;
        let expected_json_string =
            r#"{"name":"John Wick","age":99,"phones":["415 200 3475","925 200 3475"]}"#;
        assert_eq!(expected_json_string, json_string);

        Ok(())
    }

    /// This test serializes a Person object to a file and deserializes it back.
    #[test]
    fn serde_file() -> std::io::Result<()> {
        let person = Person {
            name: "John Wick".to_string(),
            age: 99,
            phones: vec!["415 200 3475".to_string(), "925 200 3475".to_string()],
        };

        let mut file = tempfile::NamedTempFile::new()?;
        serde_json::to_writer(&file, &person)?;

        file.seek(SeekFrom::Start(0))?;

        let person2: Person = serde_json::from_reader(file)?;

        assert_eq!("John Wick", person2.name);

        Ok(())
    }

    fn get_json_data() -> &'static str {
        r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#
    }
}
