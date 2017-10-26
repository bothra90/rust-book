// Boxes allow storing data on the heap rather than the stack.
// They’re most often used in these situations:
// 1. When you have a type whose size can’t be known at compile time, and you want to use a value of
// that type in a context that needs to know an exact size
// 2. When you have a large amount of data and you want to transfer ownership but ensure the data
// won’t be copied when you do so
// 3. When you want to own a value and only care that it’s a type that implements a particular trait
// rather than knowing the concrete type itself. (Trait objects)

// Boxes implement the Deref trait, which allows Box<T> values to be treated like references.
// See example: print_ref function and call to it from run().
//
// Deref coercion: Allows us to send &Box<T> where &T is required. This is allowed recursively,
// i.e. we can send &Box<Box<T>> where &T is required.

use std::fmt::Debug;
pub enum List<T>
where
    T: Debug,
{
    Nil,
    Cons(T, Box<List<T>>),
}

impl<T> List<T>
where
    T: Debug,
{
    fn print(&self) -> String {
        match *self {
            List::Nil => "\n".to_string(),
            List::Cons(ref head, ref tail) => {
                // create formatted string.
                format!("{:?} {}", *head, tail.print())
            }
        }
    }
}

pub fn run() {
    let b = Box::new(5);
    println!("b={}", b);

    // Create a cons list.
    let list: List<i32> = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );
    // We can send a reference to a Box<T> where the function expects a reference to T.
    print_ref(&list);
    // For more, see mod deref.
}

pub fn print_ref<T>(r: &List<T>)
where
    T: Debug,
{
    println!("{}", r.print());
}
