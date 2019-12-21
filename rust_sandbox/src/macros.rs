/// Rust provides a powerful macro system that allows metaprogramming. Macros look like functions
/// but they are expanded into source code that gets compiled with the rest of the program.
/// However, unlike macros in C and other languages, Rust macros are expanded into abstract
/// syntax trees, rather than string preprocessing.
///
/// Why are macros useful?
///
/// 1. Don't repeat yourself. There are many cases where you may need similar functionality in
/// multiple places but with different types. Often, writing a macro is a useful way to
/// avoid repeating code.
/// 2. Domain-specific languages. Macros allow you to define special syntax for a specific purpose.
/// 3. Variadic interfaces. Sometimes you want to define an interface that takes a variable
/// number of arguments. An example is println! which could take any number of arguments,
/// depending on the format string!.

/// Designators
///
/// The arguments of a macro are prefixed by a dollar sign $ and type annotated with a designator.
///
/// These are some of the available designators:
///
///    block
///    expr is used for expressions
///    ident is used for variable/function names
///    item
///    literal is used for literal constants
///    pat (pattern)
///    and more...
macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("You called {}()", stringify!($func_name));
        }
    };
}

create_function!(foo);

macro_rules! print_result {
    ($expression:expr) => {
        println!("{:?} = {:?}", stringify!($expression), $expression);
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_function() {
        foo();
    }

    #[test]
    fn test_print_result() {
        print_result!(1u32 + 2);
        print_result!({
            let x = 7u32;
            x * x + 2 * x - 1
        });
    }
}
