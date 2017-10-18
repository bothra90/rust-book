// Each closure instance has its own unique anonymous type: that is, even if two closures have
// the same signature, their types are still considered to be different.
//
// Closures can capture values from environment by (1) moving, (2) borrow immutably, and (3) borrow
// mutably
// (1) Encoded as FnOnce trait. FnOnce consumes the variables it captures from its enclosing scope
// (the enclosing scope is called the closure’s environment). In order to consume the captured
// variables, the closure must therefore take ownership of these variables and moves them into the
// closure when the closure is defined. The Once part of the name is because the closure can’t take
// ownership of the same variables more than once, so it can only be called one time.
// (2) Fn borrows values from the environment immutably.
// (3) FnMut can change the environment since it mutably borrows values.
//
// Question: What about values which can be copied?

use std::thread;
use std::time::Duration;
use std::collections::hash_map::HashMap;
use std::collections::hash_map::Entry;
use std::hash::Hash;

pub fn run() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout(simulated_user_specified_value, simulated_random_number);

    // Closures capture their environment:
    capture();
}

// Question: How to do this without K and V being copyable.
// Question: Why do we have to make T a template type?
// Type of @calculation is known to be Fn(K) -> V, so why use T at all?
// https://rustbyexample.com/fn/closures/anonymity.html explains why that might be necessary.
// Since Rust does not have subtypes, if we need to pass in a value to a function or store a member
// variable that implements a trait, we need to do so using generic trait bounds.
struct Cacher<T, K, V>
    where T: Fn(K) -> V,
          K: Eq + Hash + Copy,
          V: Copy,
{
    calculation: T,
    // If we want to store non-copyable values in the hashmap, we would need to store references in
    // the map, but then we would need to setup lifetimes, etc.
    values: HashMap<K, V>,
}

impl<T, K, V> Cacher<T, K, V>
    where T: Fn(K) -> V,
          K: Eq + Hash + Copy,
          V: Copy,
{
    fn new(calculation: T) -> Cacher<T, K, V> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: K) -> V {
        // Question: Is there a way to do this using HashMap::Entry?
        let entry = self.values.entry(arg);
        // This evaulates every time. Even if the element is in the map. How do we avoid that?
        // entry.or_insert((self.calculation)(arg))
        match entry {
            Entry::Occupied(o) => {
                *(o.get())
            }
            Entry::Vacant(v) => {
                let val = Box::new((self.calculation)(arg));
                v.insert(*val);
                *val
            }
        }
    }
}

fn generate_workout(intensity: i32, random_number: i32) {
    // Type annotations are not neccessary for closures, but can be added for clarity.
    let mut expensive_result = Cacher::new(|nums: i32| -> i32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        nums
    });
    if intensity < 25 {
        println!(
            "Today, do {} pushups!", expensive_result.value(intensity));
        println!(
            "Next, do {} situps!", expensive_result.value(intensity)
            );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result.value(intensity));
        }
    }
}

fn capture() {
    let x = 4;
    // Closures can capture their dynamic environment, but at a cost -> memory
    let equal_to_x = |z| z == x;
    // Functions cannot do so. Following is illegal.
    // fn equal_to_x(z: i32) -> bool { z == x }

    let y = 4;
    assert!(equal_to_x(y));
}

fn move_environment() {
    // Rust inferred earlier definition of equal_to_x to be borrowing immutably since it only
    // needed to read the value of x. However, we can explicitly move the data by a using the
    // 'move' keyword
    let x = vec![1,2,3];
    // Closures have anonymous types. The following won't compile.
    // let equal_to_x: (FnOnce(Vec<i32>) -> bool) = move |z| z == x;
    let equal_to_x = move |z| z == x;
    // This is no longer allowed.
    // println!("can't use x here: {:?}", x);
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
}
