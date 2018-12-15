// If Box is equinvalent to std::unique_ptr in C++, RC is equivalent to an std::shared_ptr.
// Rc <T> is only useful for single-threaded scenarios. For multi-threaded programs, Arc<T> will be
// required.
// A single threaded Rc type is usefull in nested data-structures such as graphs where the
// ownership of a Node could be shared by any number of parent nodes.
//
// Question: Could be enter a circular ownership situation? yes!
// Refer: https://github.com/Manishearth/rust-gc/, and
// "if a cycle of references is created, the data will be leaked" from
// https://manishearth.github.io/blog/2015/05/27/wrapper-types-in-rust-choosing-your-guarantees/
//
// There is a sister smart pointer to this one, Weak<T>. This is a non-owning, but also
// non-borrowed, smart pointer. It is also similar to &T, but it is not restricted in lifetime — a
// Weak<T> can be held on to forever. However, it is possible that an attempt to access the inner
// data may fail and return None, since this can outlive the owned Rcs. This is useful for when one
// wants cyclic data structures and other things.
//
// Rc<T> allows us to share data between multiple parts of our program for reading only, via
// *immutable references*.  If Rc<T> allowed us to have multiple mutable references too, we’d be
// able to violate one of the the borrowing rules that we discussed in Chapter 4: multiple mutable
// borrows to the same place can cause data races and inconsistencies.

use std::rc::Rc;
use crate::boxy;

use crate::dropper::CustomSmartPointer;

pub fn run() {
    // We should see the destructor of "hello" being called only once.
    let a = Rc::new(CustomSmartPointer { data: String::from("hello") });
    let _b = a.clone();
    // Listing 15-12.
    let a = boxy::List::Cons(5, Box::new(boxy::List::Cons(10, Box::new(boxy::List::Nil))));
    let _b = boxy::List::Cons(3, Box::new(a));
    // The following line would not be allowed since a has been moved into the box in the previous
    // statement.
    // let c = boxy::List::Cons(4, Box::new(a));
    //
    // With a ref counted pointer in Cons, we can do the above:
    use self::RcList::Cons;
    use self::RcList::Nil;
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    // By using Rc::clone for reference counting, we can visually distinguish between the deep copy
    // kinds of clones that might have a large impact on runtime performance and memory usage and
    // the types of clones that increase the reference count that have a comparatively small impact
    // on runtime performance and don’t allocate new memory.
    let _b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let _c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

enum RcList {
    Cons(i32, Rc<RcList>),
    Nil,
}
