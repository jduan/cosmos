/// The syntax and capabilities of closures make them very convenient for on the fly
/// usage. Calling a closure is exactly like calling a function. However, both input
/// and return types can be inferred and input variable names must be specified.
///
/// Other characteristics of closures:
/// 1. use || instead of () around input variables
/// 2. optional body delimination {} for a single expression; mandatory otherwisie
/// 3. the ability to capture the outer environment variables
///
/// Closure and functions have very similar syntax:
///
/// fn  add_one_v1   (x: u32) -> u32 { x + 1 }          // types are necessary
/// let add_one_v2 = |x: u32| -> u32 { x + 1 };         // types are not necessary
/// let add_one_v3 = |x|             { x + 1 };
/// let add_one_v4 = |x|               x + 1  ;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

pub fn run() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);

    capture_env();
    move_and_take_ownership();
}

// memorization or lazy evaluation
pub struct Cacher<T>
where
    // T is a closure type.
    T: Fn(u32) -> u32,
{
    // private fields because we want to maintain them ourselves.
    calculation: T,
    map: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            map: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.map.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.map.insert(arg, v);
                v
            }
        }
    }
}

pub fn generate_workout(intensity: u32, random_number: u32) {
    // This is how you pass a closure as a argument to Cacher::new
    // You can also annotate the closure with types although it's unnecessary:
    //  |num: u32| -> u32 { ... }
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else if random_number == 3 {
        println!("Take a break today! Remember to stay hydrated!");
    } else {
        println!(
            "Today, run for {} minutes!",
            expensive_result.value(intensity)
        );
    }
}

// This function shows the equal_to_x closure captures its surrounding environment: x in this case.
pub fn capture_env() {
    let x = 4;
    let _equal_to_x = |z: u32| z == x;
    let y = 4;
    assert_eq!(y, x);

    // This doesn't work with functions because they don't capture their surrounding environments.
    // This function doesn't compile.
    // fn equal_to_x2(z: u32) -> bool {
    //     z == x
    // }
}

pub fn move_and_take_ownership() {
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;

    // The line below doesn't compile because the ownership of x has been moved to the closure!
    // println!("Can't use x here: {:?}", x);

    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
}

/// When a function takes a closure as an input parameter, the closure's complete type
/// must be annotated using one of a few traits. In order of decreasing restriction,
/// they are:
//
//  1.  Fn: the closure captures by reference (&T)
//  2.  FnMut: the closure captures by mutable reference (&mut T)
//  3.  FnOnce: the closure captures by value (T)
pub fn apply<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

pub fn apply2<F>(f: F)
where
    F: Fn(),
{
    f();
}

pub fn apply_to_3<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(3)
}

/// On a variable-by-variable basis, the compiler will capture variables in the least restrictive
/// manner possible.
///
/// For instance, consider a parameter annotated as FnOnce. This specifies that the closure may
/// capture by &T, &mut T, or T, but the compiler will ultimately choose based on how the captured
/// variables are used in the closure.
///
/// This is because if a move is possible, then any type of borrow should also be possible. Note
/// that the reverse is not true. If the parameter is annotated as Fn, then capturing variables by
/// &mut T or T are not allowed.

// Even if "apply" takes a FnOnce, you can pass "diary" to it. "diary" is a Fn.
pub fn test_apply_1() {
    let greeting = "hello";
    let diary = || {
        println!("I said {}", greeting);
    };
    apply(diary);
}

// Even if "apply" takes a FnOnce, you can pass "diary" to it. "diary" is a FnMut.
pub fn test_apply_2() {
    let mut greeting = String::from("hello");
    let diary = || {
        greeting.push_str(", world");
        println!("I said {}", greeting);
    };
    apply(diary);
}

pub fn test_apply_3() {
    let mut greeting = String::from("hello");
    let _diary = || {
        greeting.push_str(", world");
        println!("I said {}", greeting);
    };
    // This line doesn't compile. "apply2" expects a Fn but "diary" is a FnMut.
    // apply2(diary);
}

/// You can also return closures from functions. However, anonymous closure types are, by definition, unknown, so we have to use impl Trait to return them.
///
/// The valid traits for returns are slightly different than before:
//
//    Fn: normal
//    FnMut: normal
//    FnOnce: There are some unusual things at play here, so the FnBox type is currently needed, and is unstable. This is expected to change in the future.

pub fn create_fn() -> impl Fn() {
    let text = String::from("Fn");
    let count = 0;
    move || {
        // This line doesn't compile.
        // Cannot assign to `count`, as it is a captured variable in a `Fn` closure.
        // count += 1;
        println!("calling fn #{}", count);
        println!("This is a {}", text)
    }
}

pub fn create_fn_mut() -> impl FnMut() {
    let mut text = String::from("FnMut");
    let mut count = 0;
    move || {
        count += 1;
        println!("calling fn_mut #{}", count);
        text.push_str(" hello");
        println!("This is a {}", text)
    }
}

pub fn create_fn_once() -> impl FnOnce() {
    let text = String::from("FnOnce");
    move || println!("This is a {}", text)
}

#[cfg(test)]
mod tests {
    use std::mem;

    use super::*;

    #[test]
    fn closure_syntax() {
        // Closures are anonymous, here we are binding them to references
        // Annotation is identical to function annotation but is optional
        // as are the `{}` wrapping the body. These nameless functions
        // are assigned to appropriately named variables.
        let closure_annotated = |i: i32| -> i32 { i + 1 };
        let closure_inferred = |i| i + 1;
        assert_eq!(4, closure_annotated(3));
        assert_eq!(4, closure_inferred(3));

        // A closure taking no arguments which returns an `i32`.
        // The return type is inferred.
        let one = || 1;
        assert_eq!(1, one());
    }

    /// Closures are inherently flexible and will do what the functionality requires
    /// to make the closure work without annotation. This allows capturing to flexibly
    /// adapt to the use case, sometimes moving and sometimes borrowing.
    /// Closures can capture variables:
    /// 1. by reference: &T
    /// 2. by mutable reference: &mut T
    /// 3. by value: T
    #[test]
    fn test_capturing() {
        let color = "green";
        // This closure only requires "borrowing" a reference to "color".
        let print = || {
            println!("color is {}", color);
            assert_eq!("green", color);
        };
        // Call the closure using the borrow.
        print();
        print();

        let mut count = 0;
        // A closure to increment `count` could take either `&mut count`
        // or `count` but `&mut count` is less restrictive so it takes
        // that. Immediately borrows `count`.
        // A `mut` is required on `inc` because a `&mut` reference is stored inside.
        let mut inc = || {
            count += 1;
            println!("count is {}", count);
        };
        // call the closure.
        inc();
        inc();
        assert_eq!(2, count);

        // A non-copy type.
        let movable = Box::new(3);
        let consume = || {
            println!("movable: {}", movable);
            mem::drop(movable);
        };
        // `mem::drop` requires `T` so this must take by value. A copy type
        // would copy into the closure leaving the original untouched.
        // A non-copy must move and so `movable` immediately moves into
        // the closure.
        consume();
        // "consume" consumes the variable so this can only be called once!
        // consume();

        let haystack = vec![1, 2, 3];
        // The "move" keyword explicitly forces closure to take ownership of captured
        // variables, such as "haystack".
        let contains = move |needle| haystack.contains(needle);
        assert!(contains(&1));
        assert_eq!(false, contains(&10));
        // This line doesn't compile because "haystack" has been moved to the closure.
        // haystack.push(4);
    }

    #[test]
    fn test_apply() {
        test_apply_1();
        test_apply_2();
    }

    #[test]
    fn functions_that_return_closures() {
        let fn_plain = create_fn();
        fn_plain();
        fn_plain();
        let mut fn_mut = create_fn_mut();
        fn_mut();
        fn_mut();
        let fn_once = create_fn_once();
        fn_once();
        // This line doesn't compile because you can't call fn_once more than once!
        // fn_once();
    }

    #[test]
    fn types_are_only_inferred_once() {
        let closure = |x| x;
        let s = closure(String::from("hello"));
        assert_eq!("hello", s);

        // The next line will trigger a compilation error because the first time we call
        // closure with the String value, the compiler infers the type of x and the return type
        // of the closure to be String. Those types are then locked in to the closure
        // and we get a type error if we try to use a different type with the same
        // closure.
        //
        // let n = closure(10);
    }
}
