extern crate rand; // extern is how we specify external dependencies

use rand::Rng; // Rng is a *Trait*
use std::cmp::Ordering; // Ordering is an Enum. Ordering variants are Less, Greater, and Equal
use std::io; // For reading from standard input. Why not for 'println!'?

fn main() {
    // println! is a macro, not a function call
    println!("Guess the number!");
    // let creates a variable
    // Generate a random number between 1 and 100
    // rand::thread_rng() gives us a random number generator local to the current thread of execution and seeded by the OS
    // If we leave the type of secret_number unspecfied, it can inferred from its later usage!!
    // (e.g. on comparison with guess later, Rust infers the type of secret number to be u32)
    let secret_number: u32 = rand::thread_rng().gen_range(1, 101); // gen_range function is defined in the Rng trait
    loop {
        println!("Please input your guess.");
        let mut guess = String::new(); // String type is growable and UTF-8 encoded
        // io::stdin() returns an object of type std::io::Stdin which allows us to read from
        // standard input
        io::stdin().read_line(&mut guess)  // pass a mutable reference to read_line. read_line returns a Result type, which is an Enum.
            // Result variants are Ok and Err.
            .expect("Failed to read line"); // crash on error

        // shadow variable guess
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(error) => {
                println!("Got error: {}", error);
                continue;
            }
        };
        println!("You guessed: {}", guess); // How to do formatted strings?
        // match has various "arms", to which it matches the value it is provided,
        // and executes the code associated with a matched arm.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

// From ch:09 on errors:
// To ensure that the input is actually between 1 and 100.
pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}
