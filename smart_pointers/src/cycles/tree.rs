use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

pub fn run() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
        parent: RefCell::new(Weak::new()),
    });

    // Question: Unable to understand following error.
    // Following line does not compile. Fails with error:
    //
    // 24 |     if let None = branch.parent.borrow().upgrade() {
    //    |                   ------ borrow occurs here
    // 27 | }
    //    | ^ `branch` dropped here while still borrowed
    //if let None = branch.parent.borrow().upgrade() {
    //println!("I am here");
    //}
    // Above fails only if it is the last line in the main.
    //
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}
