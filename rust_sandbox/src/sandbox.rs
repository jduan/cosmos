#[derive(Debug, Clone)]
struct Name {
    first: String,
    last: String,
}

impl Name {
    fn new(first: &str, last: &str) -> Name {
        Name {
            first: first.into(),
            last: last.into(),
        }
    }

    fn first_name(&self) -> &str {
        &self.first
    }
}

struct Person {
    name: Name,
    age: u8,
}

impl Person {
    fn new(name: Name, age: u8) -> Person {
        Person {
            name: name,
            age: age,
        }
    }

    fn name(&self) -> Name {
        self.name.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sandbox() {
        let name = Name::new("Jill", "Johnson");
        let mut jill = Person::new(name, 20);
        let name = jill.name();
        let first_name = name.first_name();
        assert_eq!("Jill", first_name);
    }
}
