#![allow(unused_variables)]
#![allow(dead_code)]
// Rust’s enums are most similar to algebraic data types in functional languages like F#, OCaml, and Haskell.
//
// V4 and V6 are "variants" of the enum. Q: How do these relate to std::variants in C++17?
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddrStruct {
    kind: IpAddrKind,
    address: String,
}

enum IpAddr {
    V4(u8, u8, u8, u8), // We can encode data within the enum itself
    V6(String),         // not all variants need to have the same data
                        // each variant can have different types and amounts of associated data.
}

// Rust enums are what are known as Algebraic Data Types (see: Haskell).
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// We can define methods over enums
impl Message {
    fn call(&self) {
        // method body would be defined here
        // Q: How do we access the values passed in the constructor?
        // Ans: using ref
        // self is borrowed.
        match *self {
            // Message is only accessible by reference, since we have an immutable reference to
            // self.
            // We use 'ref' because we can't move it out of borrowed content.
            // if the function argument was just 'self' instead of '&self', we could just write
            // Message:Write(msg) and move msg out of self.
            Message::Write(ref msg) => {
                println!("Written message: {}", msg);
            }
            Message::Move { x: _, y: _ } => {}
            Message::ChangeColor(_, _, _) => {}
            // The _ pattern will match any value, so the following is not required.
            // Message::Quit => {},
            // Otherwise, checks need to be exhaustive.
            // Using the _ placeholder for match isn't the best idea, because a newly added varian
            // might remain unchecked at different places.
            _ => {}
        }
    }
}

fn main() {
    let ip_type = IpAddrKind::V4; // Variants of an enum are namespaced under its identifier
    println!("Hello, world!");
    let home = IpAddrStruct {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddrStruct {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("Hello, World!"));
    m.call();

    // Options:
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    // Options prevent us from accessing potentially null values.
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // This is not allowed.
    // let sum = x + y;
    // Is option in Rust a monadic type?
    //
    // The if let syntax lets you combine if and let into a less verbose way to handle values that
    // match one pattern and ignore the rest
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        // Note(abhay): It makes more sense to avoid '_' as pattern in match
        _ => (),
    }
    // This is equivalent, though doesn't perform exhaustiveness check at compile time.
    if let Some(3) = some_u8_value {
        println!("three");
    }
}
