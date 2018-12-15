// Implementing Drop is equivalent to a creating a Destructor in C++.
//
// The Drop trait is included in the prelude, so we don’t need to import it
//
// We also don’t have to worry about accidentally cleaning up values still in use because that
// would cause a compiler error: the ownership system that makes sure references are always valid
// will also make sure that drop only gets called once when the value is no longer being used

use std::mem::drop;

pub fn run() {
    let _c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointers created.");
    // We can't explicitly call d.drop() since that could cause a double-free. Instead, if we want
    // to early-free resources in d, we can `move` it to the std::mem:drop function.
    drop(d);
    let _e = CustomSmartPointer { data: String::from("other other stuff") };
}

pub struct CustomSmartPointer {
    pub data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
