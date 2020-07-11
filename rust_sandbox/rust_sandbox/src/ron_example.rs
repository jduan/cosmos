/// RON is a simple readable data serialization format that looks similar to Rust syntax. It's
/// designed to support all of Serde's data model, so structs, enums, tuples, arrays, generic maps,
/// and primitive values.
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn serialize_ron() -> Result<(), Box<dyn Error>> {
        let person = Person {
            name: "John Wick".to_string(),
            age: 99,
            phones: vec!["415 200 3475".to_string(), "925 200 3475".to_string()],
        };
        let ron_s = ron::to_string(&person)?;
        let expected = r#"(name:"John Wick",age:99,phones:["415 200 3475","925 200 3475"])"#;
        assert_eq!(expected, ron_s);
        println!("{}", ron_s);

        Ok(())
    }

    #[test]
    fn deserialize_ron() -> Result<(), Box<dyn Error>> {
        let ron_data = r#"(name:"John Wick",age:99,phones:["415 200 3475","925 200 3475"])"#;
        let person: Person = ron::from_str(ron_data)?;
        assert_eq!("John Wick", person.name);
        assert_eq!(99, person.age);

        Ok(())
    }
}
