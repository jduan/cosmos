/// Rust doesn't have "inheritance", but you can define a trait as being a superset of
/// another trait.
/// Note that "super" here means superset. It doesn't mean "parent" as in superclass in Java!

trait Person {
    fn name(&self) -> String;
}

trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

struct CornellStudent {}

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

impl Person for CornellStudent {
    fn name(&self) -> String {
        String::from("John Jones")
    }
}

impl CompSciStudent for CornellStudent {
    fn git_username(&self) -> String {
        String::from("jjones")
    }
}

fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My Git username is {}",
        student.name(),
        student.university(),
        student.git_username()
    )
}

#[cfg(test)]
mod tests {
    use crate::supertrait::{comp_sci_student_greeting, CornellStudent};

    #[test]
    fn test_supertrait() {
        let john = CornellStudent {};
        assert_eq!(
            "My name is John Jones and I attend Cornell. My Git username is jjones",
            comp_sci_student_greeting(&john)
        );
    }
}
