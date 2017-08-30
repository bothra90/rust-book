#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

// Vec and String types are imported automatically. HashMaps are not.
use std::collections::HashMap;

pub fn foo() {
    // The following will not work because the std::hash::Hash and std::cmp::Eq tratis are not implemented for the Teams enum type.
    /*
    enum Teams {
        Blue,
        Yello
    };
    let mut scores = HashMap::new();
    scores.insert(Teams::Blue, 10);
    scores.insert(Teams::Yellow, 10);
    */

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yello"), 50);

    // Another way to construct hashmaps is through 'collect' methods. Collect can iterate on a vector
    // of tuples, and converts it to a hash map.
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores: Vec<u32> = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    // HashMap<_,_> annotation is needed above because it's possible to 'collect'  into different
    // data structures, and we have to tell Rust which. The key and value can, however, be
    // inferred.


    // For types implementing Copy trait, the values are copied into the hashmap. Otherwise they
    // are moved.
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point

    // Lookup:
    let color = map.get("Favorite color");
    match color {
        None => println!("No favourite color"),
        Some(c) => {
            println!("Favorite color: {}", c);
        }
    }
    for (key, value) in &map {
        println!("{}: {}", key, value);
    }

    // Mutations:
    //
    // Overwrite
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    // Insert if no value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    // Update/Insert based on value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        // or_insert returns a mutable reference (&mut V) to the value.
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
