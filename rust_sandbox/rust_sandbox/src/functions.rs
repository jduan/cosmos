pub fn add5(n: i32) -> i32 {
    let x = 5;
    // { } defines a new block, which defines its own name space for vars
    let y = {
        let x = 3;
        x + 1
    };
    println!("x is {} and y is {}", x, y);

    // this is an expression which is the return value of this function
    n + 5
}
