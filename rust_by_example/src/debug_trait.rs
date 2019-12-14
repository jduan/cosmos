use std::fmt;

/// All types can derive the "std::fmt::Debug" implementation. This is not true for
/// "std::fmt::Display", which must be manually implemented.
///
/// All std library types automatically are printable with {:?} too.
///
/// There are other traits in "std::fmt", such fmt::Binary which is used for {:b}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

/// fmt::Debug hardly looks compact and clean, so it is often advantageous to customize the output
/// appearance. This is done by manually implementing fmt::Display.
impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Person(name: {}, age: {})", self.name, self.age)
    }
}

#[cfg(test)]
mod tests {
    use crate::debug_trait::Person;

    #[test]
    fn test_debug() {
        let person = Person {
            name: "Jingjing Duan".to_string(),
            age: 99,
        };
        println!("{:#?}", person); // pretty print
        let debug_str = format!("{:?}", person); // debug print
        assert_eq!(
            debug_str,
            String::from("Person { name: \"Jingjing Duan\", age: 99 }")
        );
    }

    #[test]
    fn test_display() {
        let person = Person {
            name: "Jingjing Duan".to_string(),
            age: 99,
        };
        let display_str = format!("{}", person); // debug print
        assert_eq!(
            display_str,
            String::from("Person(name: Jingjing Duan, age: 99)")
        );
    }
}
