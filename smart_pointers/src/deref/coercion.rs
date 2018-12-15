// When the Deref trait is defined for the types involved, Rust will analyze the types and use
// Deref::deref as many times as it needs in order to get a reference to match the parameter’s type.
// This is resolved at compile time, so there is no run-time penalty for taking advantage of deref
// coercion!
use crate::deref::mybox::MyBox;

pub fn run() {
    let m = MyBox::new(String::from("foo"));
    // Type of &m is &Box<String> and it is being passed as &str
    hello(&m);
    // Without deref coercion, we would need to call:
    hello(&(*m)[..]);
}

fn hello(m: &str) {
    println!("Hello, {}", m);
}
