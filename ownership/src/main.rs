#![allow(unused_variables)]
/*
 *
 * Notes:
 * Rust ownership follows the following 3 rules:
 *  1. Any value has a variable which is its owner
 *  2. There can only be 1 owner at a time
 *  2. Value is dropped when the variable goes out of scope
 *
 * Question:
 *  - How do we do things like passing mutable references to a lot of users.
 *  e.g. how event_manager, process_util, zoo_client, etc. were shared by various
 *  objects in a system in ThoughtSpot. In other words, how do we work around the
 *  lack of "shared ownership" of a value.
 * Answer(?): via the Mutex type, which allows mutations to shared data, but the
 *  mutex API enforces ordering of mutations.
 */
fn main() {
    // String is can be mutable type, and its size is unknown at compile time.
    // It is therefore stored on the heap.
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print `hello, world!`
    // Whenever we talk of memory management, it is about memory used on the heap.
    // Memory used by a variable on the stack is automatically freed when it goes
    // out of scope.
    // Memory management could be:
    //  1. garbage collected. e.g. python
    //  2. manual; e.g. C
    //  3. automatic, but known at compile type: Rust, C++ smart ptrs

    // In rust, references to objects on the heap can't be copied, only moved.
    // Example:
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("This will not compile: {}", s1);
    println!("This is valid: {}", s2);
    //
    // To create a deep copy of a value allocated on the heap, use the clone method.
    // Question: Is clone present for all values that can be heap allocated??
    // Example:
    let s1 = String::from("hello"); // Shadows previous s1.
    let s1: String = String::from("Hello");
    // We can add Copy trait to a type to denote that value of that type should be
    // copied instead of moved when assigned to another variable.
    // Copy and Drop are traits that can be implemented by custom types
    // Copy Trait cannot be added to a type if that type, or any of its parts, has
    // impelemented the Drop trait.
    // Scalar types implement Copy.

    // Just like assignements to variables, non-Copyable values are moved when passing
    // values as function arguments or returning from functions.
    let s1 = gives_ownership(); // gives_ownership moves its return
    let s2 = String::from("hello"); // s2 comes into scope.
    let s3 = takes_and_gives_back(s2); // s2 is moved into
    // takes_and_gives_back, which also moves its return value into s3.
    // on function return, s3 goes out of scope and is dropped. s2 goes out of scope but was
    // moved, so nothing happens. s1 goes out of scope and is dropped.
    assert!(remove_vowels(format!("Hello")) == format!("Hll"));
}

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it.
    let some_string = String::from("hello"); // some_string comes into scope.
    some_string // some_string is returned and
    // moves out to the calling
    // function.
}

// takes_and_gives_back will take a String and return the same string, along with its length
fn takes_and_gives_back(a_string: String) -> (String, usize) {
    // a_string comes into scope.
    let length = a_string.len();
    // This is not legal since a_string is already moved before we get to the second element in the tuple.
    // return (a_string, a_string.len());
    (a_string, length) // a_string is returned and moves out to the calling function.
}

// Rust doc style:
/// Given a string, returns a new string that is the same as the input string
/// but does not contain any vowels from the english alphabet.
/// Example:
/// ```
/// assert!(remove_vowels(format!("Hello")) == format!("Hll"));
/// ```
fn remove_vowels(name: String) -> String {
    let mut output: String = String::new();
    // name.chars returns unicode characters.
    for c in name.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                // skip vowels.
            }
            _ => {
                output.push(c);
            }
        }
    }
    output
}
