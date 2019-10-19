pub fn run() {
    // You don't need to specify the keys in the same order
    let mut user = User {
        email: String::from("someone@example.com"),
        username: String::from("someone123"),
        active: true,
        sign_in_count: 1,
    };

    println!("user's email is {}", user.email);
    user.email = String::from("john@example.com");
    println!("user's email is {}", user.email);

    let user2 = create_user(String::from("Jack@airbnb.com"), String::from("jack"));
    println!("user2's email is {}", user2.email);

    let user3 = update_struct(user2);
    println!("user3's email is {}", user3.email);

    let mut user4 = User {
        username: String::from("Greg"),
        ..user
    };
    println!("user4's email is {}", user4.email);
    user4.email.push_str("!!!");
    println!("user4's updated email is {}", user4.email);
    // The line below won't compile because "user.email" has been moved to
    // "user4.email".
    // println!("user's email is {}", user.email);

    // This still works because user4 has its own username.
    println!("user's username is {}", user.username);

    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("Area of rectangle {:#?} is {}", rect, area(&rect));
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// "tuple structs"
// Tuple structs have the added meaning the struct name provides but don’t have names associated
// with their fields. Tuple structs are useful when you want to give the whole tuple a name and
// make the tuple be a different type from other tuples, and naming each field as in a regular
// struct would be verbose or redundant.
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn create_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        sign_in_count: 1,
        active: true,
    }
}

fn update_struct(user: User) -> User {
    User {
        email: String::from("joel@airbnb.com"),
        username: String::from("Joel"),
        // the rest fields come from "user"
        ..user
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
