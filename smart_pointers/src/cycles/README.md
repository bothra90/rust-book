Memory Leaks
------------
Preserving memory leaks is not one of Rust's guarantees.
We can see this with Rc<T> and RefCell<T>: it’s possible to create references
where items refer to each other in a cycle. This creates memory leaks because
the reference count of each item in the cycle will never reach 0, and the values
will never be dropped.
