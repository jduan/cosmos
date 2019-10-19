pub fn run() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home2 = IpAddr2::V4(String::from("127.0.0.1"));
    let loopback2 = IpAddr2::V6(String::from("::1"));

    let home3 = IpAddr3::V4(127, 0, 0, 1);
    let loopback3 = IpAddr3::V6(String::from("::1"));

    let msg1 = Message::Quit;
    msg1.call();
    let msg2 = Message::Move {x: 30, y: 50};
    msg2.call();
    let msg3 = Message::Write(String::from("hello, world"));
    msg3.call();
    let msg4 = Message::ChangeColor(255, 128, 33);
    msg4.call();
}

enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// this is better than the above
enum IpAddr2 {
    V4(String),
    V6(String),
}

// Each variant can have different types
enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn route(ip_kind: IpAddrKind) {

}

// This enum has a wide variety of types embedded in its variants.
enum Message {
    Quit,
    // This uses an anonymous struct
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// You can define methods on enums just like structs.
impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("This is a Quite message"),
            Message::Move {x: x, y: y} => println!("This is a Move (x: {}, y: {}) message", x, y),
            Message::Write(str) => println!("This is a Write message: {}", str),
            Message::ChangeColor(r, g, b) => println!("This is a ChangeColor({}, {}, {}) message", r, g, b),
        }
    }
}
