use std::ops::Range;

fn main() {
    let condition = true;
    // if conditions are expressions
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);

    let arr: [i32; 3] = [1, 2, 3];
    // Type of an iterator over an array of i32
    let r: std::slice::Iter<i32> = arr.iter();
    for i in r {
        println!("value in array: {}", i);
    }
    // When iterating over Range m..n, we get [m,n)
    for i in 0..3 {
        println!("Iterating over a range: {}", i);
    }
    // Other forms of loops are "loop" and "while"
}
