// Notes:
//
// Lifetimes:
//
// Lifetimes are just another kind of generics.
// Rather than helping us ensure that a type has the behavior we need it to have, lifetimes help
// us ensure that references are valid as long as we need them to be
//
// Ignoring elision, function signatures with lifetimes have a few constraints:
//  - any reference must have an annotated lifetime
//  - any reference being returned must have the same lifetime as an input or be static.
//
// If a function takes 2 reference arguments, both with lifetime 'a, it tells the compiler that the
// references must both live "as long as some generic lifetime 'a". This does not mean that they
// need to have the exact safe lifetime.
// Example: `longest`
// The function signature now says that for some lifetime 'a,
// the function will get two parameters, both of which are string slices that live at least as
// long as the lifetime 'a. The function will return a string slice that also will last at least
// as long as the lifetime 'a. This is the contract we are telling Rust we want it to enforce.
// Effectively, in this case Rust would need the return value to live as long as the min of the
// lifetime of x and y.
//
// Lifetime Annotations in Struct definitions:
// A lifetime annotation needs to be added on every reference in a struct's definition.
//
// Rules used by compiler to infer reference lifetimes when not explicitly annotated:
// The first rule applies to input lifetimes, and the second two rules apply to output
// lifetimes. If the compiler gets to the end of the three rules and there are still
// references that it can't figure out lifetimes for, the compiler will stop with an error.
//
// Lifetimes on function or method parameters are called input lifetimes, and lifetimes on
// return values are called output lifetimes.
//
// 1. Each parameter that is a reference gets its own lifetime parameter. In other words,
// a function with one parameter gets one lifetime parameter: fn foo<'a>(x: &'a i32), a
// function with two arguments gets two separate lifetime parameters:
// fn foo<'a, 'b>(x: &'a i32, y: &'b i32), and so on.
//
// 2. If there is exactly one input lifetime parameter, that lifetime is assigned to
// all output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32.
//
// 3. If there are multiple input lifetime parameters, but one of them is &self or &mut self
// because this is a method, then the lifetime of self is assigned to all output lifetime
// parameters. This makes writing methods much nicer.

fn main() {
    // Example: 1
    // Following would not work if y were declared after r, because in that case, y would get
    // destructed before r, resulting in a dangling reference.
    let y = 3;
    let r: &i32;
    {
        let x = 5;
        //r = &x;
        // Above is not valid since x is destroyed at the end of this block, where as r, which is
        // now a reference to x lives longer than x itself.
    }
    r = &y;
    println!("r: {}", r);

    // Example: 2
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // Example: 3
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {:?}", result);
    }

    // Example: 4
    let string1 = String::from("long string is long");
    let result: &String;
    {
        let string2 = String::from("xyz");
        // This will not work since result lives longer than string2.
        // result = longest(string1.as_str(), string2.as_str());
    }
    // println!("The longest string is {:?}", result);

    // Example: 5 (See struct ImportantExcerpt defined below)
    let novel = String::from("Call me Abhay. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // Miscellaneous.
    let x = 5;
    let y = &x;
    let z = &x;
    print_same_refs(y, z);


    // From rustbyexample.
    let x = 7;
    let y = 9;

    print_one(&x);
    print_multi(&x, &y);

    let z = pass_x(&x, &y);
    print_one(z);

    let mut t = 3;
    add_one(&mut t);
    print_one(&t);
}

// Return the longer of 2 string slices.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// `print_refs` takes two references to `i32` which have different
// lifetimes `'a` and `'b`. These two lifetimes must both be at
// least as long as the function `print_refs`.
fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

// `print_same_refs` takes two references to `i32` which have the same
// lifetimes `'a`.
fn print_same_refs<'a>(x: &'a i32, y: &'a i32) {
    println!("x is {} and y is {}", x, y);
}

// One input reference with lifetime `'a` which must live
// at least as long as the function.
fn print_one<'a>(x: &'a i32) {
    println!("`print_one`: x is {}", x);
}

// Mutable references are possible with lifetimes as well.
fn add_one<'a>(x: &'a mut i32) {
    *x += 1;
}

// Multiple elements with different lifetimes. In this case, it
// would be fine for both to have the same lifetime `'a`, but
// in more complex cases, different lifetimes may be required.
fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("`print_multi`: x is {}, y is {}", x, y);
}

// Returning references that have been passed in is acceptable.
// However, the correct lifetime must be returned.
fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 {
    x
}

//fn invalid_output<'a>() -> &'a i32 { &7 }
// The above is invalid: `'a` must live longer than the function.
// Here, `&7` would create an `i32`, followed by a reference.
// Then the data is dropped upon exiting the scope, leaving
// a reference to invalid data to be returned.

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a, 'b> ImportantExcerpt<'a> {
    // 'b is the lifetime of the object itself, where as 'a is the lifetime over which
    // ImportantExcerpt is generic.
    fn level(&'b self) -> i32 {
        3
    }
    // It can however be elided because of the elision rules.
    fn next_level(&self) -> i32 {
        3
    }
}

use std::fmt::Display;
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
