// Quote from the book:
// ...in Rust, we can’t get locking and unlocking wrong, thanks to the type system and ownership.
// Rust Mutexes however do not prevent deadlocks.

use std::sync::{Mutex, Arc};
use std::thread;

pub fn run() {
    simple();
    shared_val();
}

fn simple() {
    // Instead of mutex being a member of the data, we are wrapping the data inside a mutex.
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        // Num is a mutable reference to the data inside.
        // Even though the type of num does not look like it's a reference.
        *num = 6;
    }
    println!("m = {:?}", m);
}

fn shared_val() {
    // Just like RefCells can be used to modify values inside Rc types, Mutex can be used to modify
    // values inside Arc types.
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter: Arc<Mutex<i32>> = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}
