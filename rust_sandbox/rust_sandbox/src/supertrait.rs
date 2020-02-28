/// Rust doesn't have "inheritance", but you can define a trait as being a superset of
/// another trait.
/// Note that "super" here means superset. It doesn't mean "parent" as in superclass in Java!

pub trait Person {
    fn name(&self) -> String;
}

pub trait Race {
    fn name(&self) -> String;
}

pub trait Student: Person {
    fn university(&self) -> String;
}

pub trait Programmer {
    fn fav_language(&self) -> String;
}

pub trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

pub struct CornellStudent {}

impl Programmer for CornellStudent {
    fn fav_language(&self) -> String {
        String::from("Rust")
    }
}

impl Student for CornellStudent {
    fn university(&self) -> String {
        String::from("Cornell")
    }
}

/// A type can implement many different traits. What if two traits both require the same name?
/// For example, many traits might have a method named get(). They might even have different
/// return types!
///
/// Good news: because each trait implementation gets its own impl block, it's clear which
/// trait's get method you're implementing.
///
/// What about when it comes time to call those methods? To disambiguate between them, we
/// have to use "Fully Qualified Syntax". See the "disambiguate_traits" test below.
impl Person for CornellStudent {
    fn name(&self) -> String {
        String::from("John Jones")
    }
}

impl Race for CornellStudent {
    fn name(&self) -> String {
        String::from("Asian")
    }
}

impl CompSciStudent for CornellStudent {
    fn git_username(&self) -> String {
        String::from("jjones")
    }
}

pub fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My Git username is {}",
        student.name(),
        student.university(),
        student.git_username()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supertrait() {
        let john = CornellStudent {};
        assert_eq!(
            "My name is John Jones and I attend Cornell. My Git username is jjones",
            comp_sci_student_greeting(&john)
        );
    }

    #[test]
    fn disambiguate_traits() {
        let john = CornellStudent {};
        let race_name = <CornellStudent as Race>::name(&john);
        assert_eq!(String::from("Asian"), race_name);

        let person_name = <CornellStudent as Person>::name(&john);
        assert_eq!(String::from("John Jones"), person_name);
    }
}
