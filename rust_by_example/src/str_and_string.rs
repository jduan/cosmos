/// Prefer to pass &str than String to functions.
/// Prefer to return &str than String from functions.

#[allow(dead_code)]
fn print_me(msg: &str) {
    println!("msg is {}", msg);
}

/// When should you use 'str instead of String in structs?
/// 1. We should use a reference if our struct doesn't own the variable. We are only borrowing it
/// for a while.
/// 2. Is the String large? If it is large, then passing it by reference will save unnecessary
/// memory copy and will be a lot faster.
pub struct Person<'a> {
    name: &'a str,
}

impl<'a> Person<'a> {
    pub fn greet(&self) {
        println!("Hello, my name is {}", self.name);
    }
}

/// Sometimes a struct is really the owner of a String.
pub struct Person2 {
    pub name: String,
}

/// Use the "Into" trait to make Person2::new take anything that can be turned into a String.
/// This works for both String and &str.
impl Person2 {
    pub fn new<S>(name: S) -> Person2
    where
        S: Into<String>,
    {
        Person2 { name: name.into() }
    }
}

#[cfg(test)]
mod tests {
    use crate::str_and_string::{Person, Person2};

    #[test]
    fn test_person() {
        let person = Person {
            name: "Donald Duck",
        };
        person.greet();
    }

    #[test]
    fn test_person2() {
        let person2 = Person2::new("Herman");
        assert_eq!("Herman", &person2.name);

        // this works too thanks to the Into trait!
        let person2 = Person2::new(String::from("Herman"));
        assert_eq!("Herman", &person2.name);
    }
}
