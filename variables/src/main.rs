#![allow(unused_variables)]
fn main() {
    // x is mutable.
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    // Once transformations on a mutable variable have been performed, it might
    // be helpful to shadow the mutable type with an immutable type for the rest
    // the lifetime
    let x = x; // Shadow previous 'x'
    println!("The value of x is: {}", x);
    // Another benefit of shadowing is that the same name can be reused for a
    // different type
    let spaces = "   ";
    let spaces = spaces.len();
    // constants are different from immutable types. Advantage over simple
    // immutable variables is that constants live for the entire lifetime of a
    // program. More specifically, constants in Rust have no fixed address in memory. This is
    // because they’re effectively inlined to each place that they’re used. References to the same
    // constant are not necessarily guaranteed to refer to the same memory address for this reason.
    println!("Num spaces: {}", spaces);
    // A string literal is of type &'static str
    const STRING_CONSTANT: &'static str = "This is a constant";
    println!("A constant: {}", STRING_CONSTANT);
    // Why have an isize type? Since size is always +ve?
    let x: isize = -2;
    // String literals are immutable, so they can't be assigned to a mutable type
    // let mut_str: &'static mut str = "This is a constant";

    // one can use the format! macro to convert from string literals to heap
    // allocated string buffers.
    // example:
    // greet("Abhay");
    // This will not work, because "Abhay" is of type &'static str, where as
    // the greet function expects a String value.
    // To get around this, we can do the following:
    greet(format!("{}", "Abhay"));
    // format! is similar to fmt.Sprintf() is golang.
}

fn greet(name: String) {
    println!("Hello, {}", name);
}
