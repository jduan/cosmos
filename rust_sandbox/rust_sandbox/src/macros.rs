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
#[allow(unused_macros)]
macro_rules! create_function {
    ($func_name:ident) => {
        pub fn $func_name() {
            println!("You called {}()", stringify!($func_name));
        }
    };
}

create_function!(foo);

#[allow(unused_macros)]
macro_rules! print_result {
    ($expression:expr) => {
        println!("{:?} = {:?}", stringify!($expression), $expression);
    };
}

/// Overload
///
/// Macros can be overloaded to accept different combinations of arguments. In that regard,
/// macro_rules! can work similarly to a match block.
#[allow(unused_macros)]
macro_rules! test {
    // Arguments don't need to be separated by a comma. Any template can be used!
    ($left:expr; and $right:expr) => {{
        println!(
            "{:?} and {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left && $right
        );
        $left && $right
    }};
    ($left:expr; or $right:expr) => {{
        println!(
            "{:?} or {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left || $right
        );
        $left || $right
    }};
}

#[allow(unused_macros)]
macro_rules! my_vec {
    ($($x:expr), *) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    }
}

#[allow(unused_macros)]
macro_rules! five_times {
    ($x:expr) => {
        5 * $x
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_five_times() {
        assert_eq!(25, five_times!(2 + 3));
    }

    #[test]
    fn test_my_vec() {
        let nums = my_vec!(1, 2, 3);
        assert_eq!(vec![1, 2, 3], nums);
    }

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

    #[test]
    fn test_overload() {
        assert_eq!(false, test!(1 + 1 == 2; and 2 + 2 == 5));
        assert_eq!(true, test!(1 + 1 == 2; or 2 + 2 == 5));
    }
}
