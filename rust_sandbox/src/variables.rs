const DAY_OF_WEEK: u8 = 3;

pub fn run() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);
    println!("Day of week is {}", DAY_OF_WEEK);

    // shadow a var
    let y = 6;
    let y = y * 2 + 1;
    println!("y is {} now", y);

    // characters
    let _c = 'z';
    let _z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("heart eyed cat: {}", heart_eyed_cat);

    // tuples
    let tup = (500, 6.4, "hello");
    // destructuring
    let (_x, _y, z) = tup;
    println!("The value of z is {}", z);
    println!("The value of z is {}", tup.2);

    // arrays
    let _nums = [1, 2, 3, 4, 5];
    // explicit type
    let _nums2: [i32; 5] = [1, 2, 3, 4, 5];
    // 3 means all elements are 3; 5 means size;
    let _nums3 = [3; 5];
    let _months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
}
