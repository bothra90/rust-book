#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

pub fn foo() {
    // All values in a vector must be of the same type.
    // Since we are creating an empty vector, we have to tell rust what the type of the values is going
    // to be
    let v: Vec<i32> = Vec::new();
    // We can also create a vector using the vec! macro. Here since we are providing the initial
    // values for v, we don't have to specify its type - it is inferred to be Vec<i32>
    let v = vec![1, 2, 3];
    // Mutating a vector.
    let v = vec![1];
    // This is not allowed since v is immutable.
    // v.push(6);
    // Let's try again
    let mut v = vec![1];
    v.push(2);
    v.push(3);
    // This will not work
    // v.push("foo");

    // Contents of a vector are dropped when the vector is dropped.
    {
        let v = vec![1, 2, 3, 4];
        // do stuff with v
        //
    } // <- v goes out of scope and is freed here

    // Accessing elements
    let v = vec![1, 2, 3, 4, 5];
    // 2 ways.
    let third: &i32 = &v[2];
    let third: Option<&i32> = v.get(2);
    // Difference in usage:
    // let dne: &i32 = &v[100]; // will panic at runtime
    let dne: Option<&i32> = v.get(100); // will not panic at runtime
    match dne {
        Some(&v) => println!("got {}", v),
        None => println!("no value found"),
    }
    let x: i32 = 2;
    let y: &i32 = &x;
    // Previous x was allocated on the stack, so its reference is still valid
    println!("value: {}", y);

    // This is however not allowed!!
    //let y:&i32;
    //{
    //let x = 5;
    //y = &x
    //}
    //println!("value: {}", y);

    // can we take slices of vectors?
    let mut v = vec![1, 2, 3];
    let slice = &v[1..2];
    // v.push(6); Not allowed by borrow checker!

    // When we want to store elements of different (but known) types in a vector, we can wrap them
    // inside an enum and then create a vector of enums!!
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
