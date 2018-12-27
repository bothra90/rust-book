// Notes:
//
// Generics and Monomorphization:
// Rust accomplishes this by performing monomorphization of code using generics
// at compile time. Monomorphization is the process of turning generic code
// into specific code with the concrete types that are actually used filled in.
// The compiler looks at all the places that generic code is called and generates
// code for the concrete types that the generic code is called with.
//
// Traits:
// One restriction to note with trait implementations: we may implement a trait on a
// type as long as either the trait or the type are local to our crate. For example,
// we cannot implement the Display trait on the Vec struct, since both are defined in the
// standard library, and not local to our trait.
// This restriction is part of what's called the orphan rule, which you can look up if
// you're interested in type theory. Briefly, it's called the orphan rule because the
// parent type is not present. Without this rule, two crates could implement the same
// trait for the same type, and the two implementations would conflict: Rust wouldn't
// know which implementation to use. Because Rust enforces the orphan rule, other
// people's code can't break your code and vice versa.
//
// Rust traits can have default implementations. See DefaultSerializable below for an example.
// Default implementations are allowed to call the other methods in the same trait, even if
// those other methods don't have a default implementation. In this way, a trait can provide
// a lot of useful functionality and only require implementers to specify a small part of it.
//
// Traits are similar to golang interfaces.
// Due to go's "duck typing", a struct can implement an interface without explicitly declaring so.
// This allows for easy creation of abstractions after-the-fact of creating the structs - i.e. we
// can declare an interface such that an existing type (even from a third-party library) comes to
// implement that interface by virtue of the functions implemented on that type.
// We can do the same with Rust structs as well. If a type in a third-party library implements some
// function, we can create a Trait with that function, and then in our library declare that type to
// implement the trait we defined. This is in-line with the law earlier stated.
//
// Traits and trait bounds let us write code that uses generic type parameters in order to reduce
// duplication, but still specify to the compiler exactly what behavior our code needs the generic
// type to have. Because we've given the trait bound information to the compiler, it can check
// that all the concrete types used with our code provide the right behavior. In dynamically typed
// languages, if we tried to call a method on a type that the type didn't implement, we'd get an
// error at runtime. Rust moves these errors to compile time so that we're forced to fix the
// problems before our code is even able to run. Additionally, we don't have to write code that
// checks for behavior at runtime since we've already checked at compile time, which improves
// performance compared to other languages without having to give up the flexibility of generics.
//
fn main() {
    println!("Hello, world!");
    // Following are OK since x and y are of the same type: i32 and f64 respectively.
    let _integer = Point { x: 5, y: 10 };
    let _float = Point { x: 1.0, y: 4.0 };
    // However, the following does not compile:
    //  let wont_work = Point { x: 5, y: 4.0 };

    // DoublePoints allow x and y to have different types.
    let p1 = DoublePoint { x: 5, y: 10.4 };
    let p2 = DoublePoint { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    // Since tweet has its own summary method, that is preferred over the 2 from the traits
    // mixed-in. We can call the trait methods by the following syntax.
    println!("1 new tweet: {}", tweet.summary());
    println!("{}", <Tweet as DefaultSummarizable>::summary(&tweet));
    println!("{}", <Tweet as Summarizable>::summary(&tweet));
}

// Structs can be generic over types.
// Point has 2 fields `x` and `y`, both of the same type.
struct Point<T> {
    x: T,
    y: T,
}

// Enums can also be generic over types.
enum Option<T> {
    Some(T),
    None,
}

// Option and Result are generic traits provided by the standard library.
enum Result<T, E> {
    Ok(T),
    Err(E),
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

struct DoublePoint<T, U> {
    x: T,
    y: U,
}

impl<T, U> DoublePoint<T, U> {
    // Generic type parameters in a struct definition aren't always the same
    // generic type parameters you want to use in that struct's method signatures.
    fn mixup<V, W>(self, other: DoublePoint<V, W>) -> DoublePoint<T, W> {
        DoublePoint {
            x: self.x,
            y: other.y,
        }
    }
}

// Largest takes a slice over a list of objects of type T and returns the largest.
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            // T has to be comparable
            largest = item;
        }
    }
    largest // T has to implement the copy trait because we want to return a value instead of a reference.
}

// Custom traits:
pub trait Summarizable {
    fn summary(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summarizable for NewsArticle {
    fn summary(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summarizable for Tweet {
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub trait MultiFunctionalTrait {
    fn foo(&self);
    fn bar(&self) -> &str;
}

pub struct FooBar {
    x: String,
}

impl MultiFunctionalTrait for FooBar {
    fn foo(&self) {
        println!("Some message");
    }
    fn bar(&self) -> &str {
        &self.x
    }
}

// Unlike implementing normal functions on a struct, we cannot split its implementation of a trait
// into multiple blocks. Following is not legal:
//impl MultiFunctionalTrait for FooBar {
//fn bar(&self) -> &str {
//&x
//}
//}

// Traits can have default implementatins.
pub trait DefaultSummarizable {
    fn summary(&self) -> String {
        String::from("(Read more...)")
    }
}

// We can't have the following, since Tweet already has the summary method via the Summarizable
// trait.
impl DefaultSummarizable for Tweet {}
// we can have it, if Tweet itself implements the summary functions.
impl Tweet {
    fn summary(&self) -> String {
        String::from("This is a tweet")
    }
}

// Traits can provide default implementations for some functions which make use of trait methods
// w/o default implementation.
pub trait SummarizableDefault {
    fn author_summary(&self) -> String;

    fn summary(&self) -> String {
        format!("(Read more from {}...)", self.author_summary())
    }
}

// Trait bounds:
// functon notify is generic on any type T which implements the Summarizable trait.
pub fn notify<T: Summarizable>(item: T) {
    println!("Breaking news! {}", item.summary());
}

pub fn notify_2018(item: impl Summarizable) {
    println!("Breaking news! {}", item.summary());
}

trait Trait {}

struct Foo {}
struct Bar {}

impl Trait for Foo {}
impl Trait for Bar {}

fn ret_trait(x: bool) -> Box<dyn Trait> {
    if x {
        Box::new(Foo {})
    } else {
        Box::new(Bar {})
    }
}

/*
 * Following would not be allowed, because `impl` requires its type to be concrete, not dynamic
 *
 fn ret_trait(x: bool) -> impl Trait {
     if x {
        Foo{}
     } else {
        Bar{}
     }
 }
 *
 * */
