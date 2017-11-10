use std::thread;
pub fn run() {
    simple();
    capture();
}

fn simple() {
    let handle = thread::spawn(|| for i in 1..10 {
        println!("hi number {} from the spawned thread!", i);
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
    }
    handle.join();
}

fn capture() {
    let v = vec![1, 2, 3];
    let x = 5;
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    // drop(v); // This is not valid once v has been moved intot he closure.
    // x can however be used because it is not moved. Why is it not moved?
    println!("{}", x);
    handle.join();
}
