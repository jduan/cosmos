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

/// Type alias can be created for a lot of other things! The type statement can be used to give
/// a new name to an existing type. Types must have "CamelCase" names, or the compiler will raise
/// a warning. The exception to this rule are the primitive types: usize, f32, etc.
///
/// The main use of type aliases is to reduce boilerplate; for example, the "IoResult<T>" type is
/// an alias for the "Result<T, IoError>" type.
pub type NanoSecond = u64;
pub type Inch = u64;

#[cfg(test)]
mod tests {
    use crate::type_alias::{add_or_subtract, Inch, NanoSecond, Operations};

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

    #[test]
    fn test_aliasing() {
        let elapsed: NanoSecond = 5;
        let inches: Inch = 200;
        assert_eq!(5, elapsed);
        assert_eq!(200, inches);
    }
}
