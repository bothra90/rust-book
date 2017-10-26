// The characteristics that distinguish a smart pointer from an ordinary struct are that smart
// pointers implement the Deref and Drop traits. The Deref trait allows an instance of the smart
// pointer struct to behave like a reference so that we can write code that works with either
// references or smart pointers. The Drop trait allows us to customize the code that gets run when
// an instance of the smart pointer goes out of scope

extern crate smart_pointers;

use smart_pointers::*;

fn main() {
    let mut x = 5;
    {
        // y is a simple mutable reference to x.
        let y = &mut x;
        *y += 1
    }
    assert_eq!(6, x);
    println!("Hello, world!");

    // Box.
    boxy::run();
    // Deref.
    deref::run();
    // Drop.
    dropper::run();
    // RC.
    rc::run();
    // RefCell.
    refcell::run();
    // Reference cycles.
    cycles::run();
}
