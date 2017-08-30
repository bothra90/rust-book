#![allow(unused_variables)]

use std::ops::Range;

fn main() {
    /*
     * Block comments
     */
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("Tuple[1]: {}", tup.1);
    //  once declared, arrays cannot grow or shrink in size.
    //  The type of an array encodes both the type and the number of elements in the array.
    //  Unlike arrays, vectors are allowed to grow or shrink
    // A rust program panics when accessing an element out-of-bounds of an array
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // Range type. Written as: m...n
    // Type is std::ops::Range<{integer}>
    let b: Range<i32> = 2..4;

    // We cannot add an unsiged integer to a signed integer, unlike C or C++
    let a: i32 = 5;
    //let b: u32 = 7;
    //let c = a + b;
    //println!("{}", c);
}
