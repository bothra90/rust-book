use std::io;
use std::io::Read;
use std::fs::File;
use std::fs::remove_file;
use std::io::ErrorKind;

// Notes:
//
// Errors in Rust are of 2 types.
//  1. Recoverable (via Result type)
//  2. Unrecoverable (via panic! macro which stops execution)
//
// Panic!:
// When the panic macro is encountered, the program stops execution, unwinds and cleans up the
// stack, and then exits.
// The stacktrace shows the following:
//    0: std::sys::imp::backtrace::tracing::imp::unwind_backtrace
//    1: std::panicking::default_hook::{{closure}}
//    2: std::panicking::default_hook
//    3: std::panicking::rust_panic_with_hook
//    4: std::panicking::begin_panic
//
// Question: What is the default hook? How can we insert a custom hook? Is it similar to Golang's
// recover?
//
// In order to directly exit on panic! without the stack unwinding, we have to add the following to
// the config.toml file:
// ```
//  [profile.release]
//  panic = 'abort'
// ```
fn main() {
    // Example: 1 (commented out to prevent crash)
    // panic("hello, world!");

    // Example 2: Reading a file that does not exit.
    // f is of type Result<std::fs::file, std::io::Error>
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Failed to open file hello.txt, error: {}", error);
        }
    };

    // Example: 3
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        // the if condition after ther Err match specifies a "match guard"
        // ref is need to ensure that error is not moved into this condition, but only referenced
        // by it.
        // In the context of a pattern, & matches a reference and gives us its value, but ref
        // matches a value and gives us a reference to it.
        Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
        },
        Err(error) => panic!("There was a problem opening the file: {:?}", error),
    };


    // Example: 4
    // Unwrap returns the contents of Ok if the operation succeeds, otherwise panics.
    let f = File::open("hello.txt").unwrap();

    // Example: 5.
    // remove_file("hello.txt").unwrap();
    // expect is just like unwrap, except that it panics with the given msg.
    let f = File::open("hello.txt").expect("Failed to open file");

    // Example: 6. Propagating errors.
    let name = read_username_from_file().expect("Failed to get username");
    println!("Username is {:?}", name);

    // Example: 7 (Propagating errors with '?')
    let name = read_username_from_file_short().expect("Failed to get username");
    println!("Username is {:?}", name);
    let name = read_username_from_file_shorter().expect("Failed to get username");
    println!("Username is {:?}", name);


    // Miscellaneous.
    // Following is not allowed because lifetime of x mismatches with function parameter lifetime.
    // let x: String = String::from("foo");
    // panic(x);
    panic("foo");
}

// Need to add 'static lifetime to str, because the argument to panic! should have a 'static
// lifetime.
fn panic(msg: &'static str) {
    // println! doesn't work even if we have a static string -> it needs a literal. Why?
    // println!(msg);
    panic!(msg);
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_short() -> Result<String, io::Error> {
    // expression ending with '?' causes function to return immediately if the result is an error,
    // otherwise assigns value from Ok to the lhs.
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_shorter() -> Result<String, io::Error> {
    let mut s = String::new();
    // Function chaining due to early returns.
    // This looks great!!
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
