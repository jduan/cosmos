/// The syntax and capabilities of closures make them very convenient for on the fly
/// usage. Calling a closure is exactly like calling a function. However, both input
/// and return types can be inferred and input variable names must be specified.
///
/// Other characteristics of closures:
/// 1. use || instead of () around input variables
/// 2. optional body delimination {} for a single expression; mandatory otherwisie
/// 3. the ability to capture the outer environment variables
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
struct Cacher<T>
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

fn generate_workout(intensity: u32, random_number: u32) {
    // This is how you pass a closure as a argument to Cacher::new
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

fn types_are_inferred_once() {
    let closure = |x| x;
    let s = closure(String::from("hello"));

    // The next line will trigger a compilation error because The first time we call
    // closure with the String value, the compiler infers the type of x and the return type
    // of the closure to be String. Those types are then locked in to the closure in
    // closure, and we get a type error if we try to use a different type with the same
    // closure.
    // let n = closure(10);
}

// This function shows the equal_to_x closure captures its surrounding environment: x in this case.
fn capture_env() {
    let x = 4;
    let equal_to_x = |z: u32| z == x;
    let y = 4;
    assert_eq!(y, x);

    // This doesn't work with functions because they don't capture their surrounding environments.
    // This function doesn't compile.
    // fn equal_to_x2(z: u32) -> bool {
    //     z == x
    // }
}

fn move_and_take_ownership() {
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;

    // The line below doesn't compile because the ownership of x has been moved to the closure!
    // println!("Can't use x here: {:?}", x);

    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
}

#[cfg(test)]
mod tests {
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
}
