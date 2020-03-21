#[derive(Debug, Clone)]
pub struct Name {
    first: String,
    last: String,
}

impl Name {
    pub fn new(first: &str, last: &str) -> Name {
        Name {
            first: first.into(),
            last: last.into(),
        }
    }

    pub fn first_name(&self) -> &str {
        &self.first
    }
}

pub struct Person {
    name: Name,
    pub age: u8,
}

impl Person {
    pub fn new(name: Name, age: u8) -> Person {
        Person { name, age }
    }

    pub fn name(&self) -> Name {
        self.name.clone()
    }
}

pub fn two_words() -> (String, String) {
    ("fellow".to_string(), "Rustaceans".to_string())
}

/// Concatenate `suffix` onto the end of `prefix`.
pub fn join_words(prefix: &mut String, suffix: &str) {
    prefix.push(' '); // separate the words with a space
    prefix.push_str(suffix);
}

// Challenge: Convert `join_words` to use borrowing, not ownership.
// The new function should mutate `prefix` in place, and should not
// take ownership of `suffix`.
//
// Hint: If you'd like a hint as to how to proceed, open
// <http://intorust.com/hint/mutable_borrow_1/>.

// Question: Now that you've converted `join_words`, what happens if you
// call `join_words` using the same string for `prefix` and `suffix`?
// Why?

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sandbox() {
        let name = Name::new("Jill", "Johnson");
        let jill = Person::new(name, 20);
        let name = jill.name();
        let first_name = name.first_name();
        assert_eq!("Jill", first_name);
    }

    #[test]
    fn test_mutability() {
        let mut buffer = String::from("Rustacean");
        for i in 0..buffer.len() {
            let slice = &buffer[i..];
            println!("slice: {:?}", slice);
        }

        buffer.push_str("s");
        println!("buffer itself: {:?}", buffer);
    }
    #[test]
    fn test_mutability2() {
        let (mut str1, str2) = two_words();
        join_words(&mut str1, &str2);
        assert_eq!("fellow Rustaceans", str1);
    }
}
