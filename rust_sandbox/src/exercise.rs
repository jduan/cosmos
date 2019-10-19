pub fn run() {
    println!("fahrenheit 99 in celsius is {}", f2c(99.0));
    println!("The 10th Fibonacci number is {}", nth_fib(10));
}

// Convert fahrenheit to celsius
fn f2c(x: f32) -> f32 {
    (x - 32.0) * 5.0 / 9.0
}

// generate the nth fibonacci number
fn nth_fib(n: u32) -> u32 {
    if n == 1 {
        0
    } else if n == 2 {
        1
    } else {
        let mut ret: u32 = 0;
        let mut x = 0;
        let mut y = 1;

        for i in (2..n) {
            ret = x + y;
            x = y;
            y = ret;
        }

        ret
    }
}
