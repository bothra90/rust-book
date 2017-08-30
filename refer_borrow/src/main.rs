#![allow(unused_variables)]
// Code for understanding references and borrowing
// http://doc.rust-lang.org/beta/book/second-edition/ch04-02-references-and-borrowing.html
//
// The Rules of References
//
// 1. At any given time, you can have either but not both of:
//    i. One mutable reference.
//    ii. Any number of immutable references.
//  This is like having compile-time read-write locks.
// 2. References must always be valid.
fn main() {
    // Shared references.
    let s = format!("Hello");
    // refr is a shared reference to a string buffer.
    let refr = &s;
    // We can call print twice with refr, because shared reference types are Copy types.
    print(refr);
    print(refr);

    let x = 5;
    println!("Hello, world!");
    // Create string
    let s: String = String::from("Hello, World!");
    let size: usize = calculate_length(&s);
    println!("size: {}", size);

    // When a mutable object is created, it doesn't mean it's always mutable.
    // For example, if the variable has been borrowed, it is immutable during the lifetime of the
    // borrow. It's important to remember that the lifetime of an object is different from its last
    // point of use.
    let mut name = format!("Fello rustacean!");
    let r: &String = &name; // r is an immutable borrow of name.
    print(r);
    // This is not allowed:
    // name.push('x');
    print(r);
    // name.push('x') is still now allowed because `r` is still is still not destroyed.
    // This can be handled in the following way:
    let mut name = format!("Fello rustacean!");
    {
        let r: &String = &name; // r is an immutable borrow of name.
        print(r);
        print(r);
    } // <-- borrow ends here.
    // This is now allowed.
    name.push('x');

    // Similarly, we cannot make an immutable reference to an object while a mutable reference
    // exists.
    let mut name = String::from("hello rustacean");
    let r = &mut name;
    add_name(r);
    add_name(r);
    // Following is not allowed.
    // println!("{}", &name);
    // Following is also not allowed.
    // let r2 = &mut name;
    // The followint 2 blocks are equivalent
    // BLOCK 1:
    // add_name(&mut name);
    //
    // BLOCK 2:
    // {
    //   let r = &mut name;
    //   add_name(r)
    // } <-- r goes out of scope.


    // Only mutable strings can be passed
    let mut s: String = String::from("Hello, ");
    // Both the value and it's reference have to be annotated with 'mut'
    add_world(&mut s);

    // Only 1 mutable borrow can exist at any point in time.
    // As long as a mutable borrow exists, no other reference can exist, mutable or immutable
    // 1 or more immutable borrows can exist simultaneously.
    // If an immutable borrow exist, a mutable borrow cannot be created.
    let mut x = 5;
    let y = &mut x;
    // This will fail
    // let z = &mut x;
    // This will also fail.
    // let z = &x;
    //
    // Through allowing at most 1 mutable borrow, Rust prevents:
    //  1. Data races
    //  2. Use after free -> might manifest as iterator invalidation, or when a string slice refers
    //     to a portion of memory that might no longer have the string if the underlying string got
    //     moved due to a mutaiton (e.g. appending characters which would not fit into current
    //     capacity)
    //  3. Double free

    // Through lifetimes, Rust prevents Dangling pointers.
    // Lifetimes ensures that references are always valid, i.e. values they refer to will not go
    // out of scope (be dropped).
    dangle();
}

fn add_name(name: &mut String) {
    name.push_str("abhay ");
}

// If a function is going to modify a non-mutable reference argument in its body, that argument name has
// to be prefixed with the `mut` keyword. This tells the compiler that we will be mutating the
// value of the argument variable in the body. If the type was a mutable-reference, we would still
// need the mut keyword if we wanted to reassign the variable to some other mutable reference.
fn add_name_move(mut name: String) {
    name.push_str("abhay ");
}

fn print(name: &String) {
    println!("{}", name)
}

fn calculate_length(s: &String /* s is a reference to a String */) -> usize {
    s.len()
} // s goes out of scope. But since it doesn't own what it refers to, nothing happens.

fn add_world(s: &mut String) {
    s.push_str("World!")
}

fn dangle() -> &'static str {
    return "hello";
    // If the return type of the function was &String, the following would not be allowed.
    // let s = String::from("hello");
    // &s
    // s goes out of scope here. We therefore cannot return a reference to it
}
