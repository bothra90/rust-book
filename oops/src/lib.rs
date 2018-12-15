/*
 * Limitations:
 * Object Safety Is Required for Trait Objects

You can only make object-safe traits into trait objects. Some complex rules govern all the
properties that make a trait object safe, but in practice, only two rules are relevant. A trait is
object safe if all the methods defined in the trait have the following properties:

    1. The return type isn’t Self.
    2. There are no generic type parameters.

The Self keyword is an alias for the type we’re implementing the traits or methods on. Trait
objects must be object safe because once you’ve used a trait object, Rust no longer knows the
concrete type that’s implementing that trait. If a trait method returns the concrete Self type, but
a trait object forgets the exact type that Self is, there is no way the method can use the original
concrete type. The same is true of generic type parameters that are filled in with concrete type
parameters when the trait is used: the concrete types become part of the type that implements the
trait. When the type is forgotten through the use of a trait object, there is no way to know what
types to fill in the generic type parameters with.

Question: Is this similar to type-erasure in Java?
 *
 * */
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // A trait object points to an instance of a type that implements the trait we specify.
    // We can use reference (&) or a Box type. It has to be a pointer type.
    // pub components: Vec<&'a dyn Draw>,
    /*
     * This works differently than defining a struct that uses a generic type parameter with trait
     * bounds. A generic type parameter can only be substituted with one concrete type at a time,
     * whereas trait objects allow for multiple concrete types to fill in for the trait object at
     * runtime.
     *
     * When we use trait objects, Rust must use dynamic dispatch. The compiler doesn’t know all the
     * types that might be used with the code that is using trait objects, so it doesn’t know which
     * method implemented on which type to call.
     *
     * So, this is a trade-off between flexibility and runtime performance.
     */
    pub components: Vec<Box<Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

/*
Implementation of Screen using generics with trait bounds would look like this:
If we are sure are elements will be of the same time, this is more preferable because of compile time monomorphization.
Trait objects, on the other hand, have runtime performance implications.

pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
    where T: Draw {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
 */

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}
