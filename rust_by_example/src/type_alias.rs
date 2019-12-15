/// Type alias can be created for a lot of other things!
pub enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    pub fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            // Self here means VeryVerboseEnumOfThingsToDoWithNumbers
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

// Create a type alias
pub type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

pub fn add_or_subtract(op: Operations, x: i32, y: i32) -> i32 {
    match op {
        Operations::Add => x + y,
        Operations::Subtract => x - y,
    }
}

#[cfg(test)]
mod tests {
    use crate::type_alias::{add_or_subtract, Operations};

    #[test]
    fn test_run() {
        assert_eq!(7, Operations::Add.run(3, 4));
        assert_eq!(-1, Operations::Subtract.run(3, 4));
    }

    #[test]
    fn test_add_or_subtract() {
        assert_eq!(7, add_or_subtract(Operations::Add, 3, 4));
        assert_eq!(-1, add_or_subtract(Operations::Subtract, 3, 4));
    }
}
