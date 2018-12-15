#![allow(unused_variables)]
#![allow(unused_mut)]
/*
 * Slices reference a contiguous sequence of elements in a collection.
 * They themselves do not have ownership of the referrenced data.
 * They are like fat pointers, since they have both a pointer, and a length of the data they are
 * pointing to.
 */

fn main() {
    println!("Hello, world!");
    let mut s: String = String::from("hello, world!");
    let loc = first_word(&s);
    println!("first word ends at {}", loc);
    s.truncate(2);
    println!("now the string is \"{}\"", s);
    // println!("reference is now invalid: {}", s.chars().nth(loc).unwrap());

    // A string slice is a reference to a part of a string in memory.
    // They are great when we want to avoid the expensive copying of strings when performing
    // various string operations.
    // So String object = len + capacity + ref to data in memory,
    // whereas &str = ref to data in memory + len
    // This is very similar to string slices in golang.
    let mut s: String = String::from("hello, world!");
    // The type that signifies “string slice” is written as &str
    // A string reference contains a reference to the location in the String
    // where it starts, and the length of the refernece. This enables us to iterate over the
    // characters of a slice.
    let first_word: &str = first_word_slice(&s);
    for c in first_word.chars() {
        println!("{}", c);
    }
    // Following is a runtime error.
    // let randome_len: &str = &s[..15];
    //
    // example when string references prove to be very efficient.
    let sentence: String = String::from("The cat is sat on the mat");
    for word in sentence.split(' ') {
        println!("{}", word);
    }
    // In the above example, Rust does not make any copies from the string. Instead, on each
    // iteration it simply returns a slice over the next word in the string, saving a lot of
    // copying/allocations.

    // We can no longer truncate the string
    // s.truncate(2);  // -> Compiler error.
    // string literals are also &str type variables.
    // It’s a slice pointing to that specific point of the binary. This also explains why
    // variables which point to string literals are immutable.
    // Note from the future: They also have the 'static lifetime.
    let s: &str = "Hello, World!";
    // We can have slices of arrays too!
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // slice now holds an immutable reference to a
    let slice: &[i32] = &a[1..3];
    println!("Array slice first item: {}", slice[0]);
    // However, the index is not checked, which could cause a panic at runtime.
    // println!("Array slice first item: {}", slice[10]);
    let a = [1, 3, 5];
    let v = vec![2, 4, 6];
    let s = String::from("fun");
    let sl = "world";
    // We can pass a ref to an array, ref to a vector, or ref to a vector slice to a function
    // taking a slice.
    printSlice(&a);
    printSlice(&v);
    printSlice(&v[1..]);
    // We can pass both &String and &str to a function taking a string slice.
    printStr(&s);
    printStr(sl);
}

/*
 * The problem with this approach is that if the string itself is mutable, an index to
 * a character might be become invalid.
 */
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // this is same as &s[..i]
            return &s[0..i];
        }
    }
    // Equivalent: &s[0..] and len = s.len(); &s[..len]
    &s[..]
}

fn printSlice<T: std::fmt::Display>(slice: &[T]) {
    for i in slice {
        println!("{}", i);
    }
}

fn printStr(s: &str) {
    println!("{}", s);
}
