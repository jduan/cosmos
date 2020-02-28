/// Diverging functions never return. They are marked using !, which is an empty type.
pub fn foo() -> ! {
    panic!("This can never returns!");
}

/// As opposed to all the other types, the "empty type" cannot be instantiated, because the
/// set of all possible values this type can have is empty. Note that, it is
/// different from the () type, which has exactly one possible value.
pub fn some_fn() {
    #[allow(clippy::unused_unit)]
    ()
}

/// Although this might seem like an abstract concept, it is in fact very useful
/// and often handy. The main advantage of this type is that "it can be cast to any
/// other one" and therefore used at places where an exact type is required.
pub fn sum_odd_numbers(upto: u32) -> u32 {
    let mut sum = 0;
    for i in 1..upto {
        let additional = if i % 2 == 1 {
            i
        } else {
            // the "continue" expression does not return a "u32" but it is still fine,
            // because it never returns and therefore does not violate the type
            // requirements of the match expression.
            continue;
        };
        sum += additional;
    }

    sum
}

/// The "empty type" is also the return type of functions that loop forever (e.g. loop {})
/// like network servers or functions that terminates the process (e.g. exit()).

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_foo() {
        foo();
    }

    #[test]
    #[should_panic]
    fn test_empty_type() {
        let a: () = some_fn();
        println!("This function returns {:?} and you can see this line", a);

        let _x: ! = panic!("This call never returns");
        // println!("You will never see this line.");
    }

    #[test]
    fn test_sum_odd_numbers() {
        let sum = sum_odd_numbers(10);
        assert_eq!(25, sum);
    }
}
