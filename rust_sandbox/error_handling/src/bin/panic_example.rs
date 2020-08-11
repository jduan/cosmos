use std::env;

// I like to think of this style of error handling as similar to a bull running through a china
// shop. The bull will get to where it wants to go, but it will trample everything in the process.
fn main() {
    let mut argv = env::args();
    let arg: String = argv.nth(1).expect("expect an argument");
    let n: i32 = arg.parse().expect("expect an integer");
    println!("{}", 2 * n);
}
