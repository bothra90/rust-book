use std::sync::mpsc; // mpsc == multipe-provider single-consumer
use std::thread;

pub fn run() {
    nowait();
    single_sender();
    multi_sender();
}

fn nowait() {
    // Shows that sender doesn't block for a receiver.
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        println!("done sending");
    });
    handle.join();
}

fn single_sender() {
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // The following will cause a compilation failure since `val` has been moved to the
        // receiver. This is very profound, since we are passing ownership across threads!
        // println!("Sent: {}", val);
    });
    // drop(rx); This would cause a panic in the sender if the send has not already completed.
    let received = rx.recv().unwrap();
    // recv() blocks for a sender. try_recv does not.
    println!("Got: {}", received);
    // handle.join();
}

fn multi_sender() {
    use std::time::Duration;
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    // First sender.
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    // second sender.
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
