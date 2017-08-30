#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

pub fn foo() {
    // strings are implemented as a collection of bytes plus some methods to provide useful
    // functionality when those bytes are interpreted as text.

    // the 'str' type represents a string slice, and is usually seen in its borrowed form &str
    // str is a type in the langugage itself
    // String type is provided by the standard library

    // Both String and string slices are UTF-8 encoded.
    let mut s = String::from("foo bar");
    {
        let slice: &str = &mut s[0..4];
        // Can't do the following??
        // slice[2] = 'x';
    }
    println!("new string: {}", s);

    // 2 ways of creating String objects from string literals are equivalent
    let s = "foo bar".to_string();
    let s = String::from("foo bar");

    // Since strings are utf-8, the following are all valid
    let hello = "السلام عليكم";
    let hello = "Dobrý den";
    let hello = "Hello";
    let hello = "שָׁלוֹם";
    let hello = "नमस्ते";
    let hello = "こんにちは";
    let hello = "안녕하세요";
    let hello = "你好";
    let hello = "Olá";
    let hello = "Здравствуйте";
    let hello = "Hola";

    // When taking string slices, we only read the bytes specified by the range. If the range of
    // bytes is cannot be decoded to UTF-8 code points, an attemp to do so might result in an
    // error.
    let hello = "नमस्ते";
    let s = &hello[..3];
    // This will only print न instead of नमस्.
    println!("I say: {}", s);
    // If we instead give the range of [0..2], the programs panics with the error:
    // 'byte index 2 is not a char boundary; it is inside 'न' (bytes 0..3) of `नमस्ते'
    // let s = &hello[..2];
    // println!("{}", s);

    // String mutations

    // Push a string slice
    let mut hello = String::from("hello");
    hello.push_str("foo");

    // Push a character (unicode code point)
    let mut hello = "hol".to_string();
    hello.push('न');
    println!("hol is now: {}", hello);

    // Concat strings
    let hello = "hello,".to_string();
    let world = String::from(" world!");
    let hello_world = hello /* moved */ + &world;
    // We can't add 2 string values. The + operator takes a string reference as its argument.
    // Does this lead to new space allocation to store the new string?
    // hello = hello + world; is not allowed
    // CAn only add a &str to a String. &String gets coerced into a &str using deref coercion
    // (something like converting &s to &s[..].
    println!("{}", hello_world);

    // Concatenating multiple strings
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // format! macro is similar to println. Does not take ownership of its arguments
    let s = format!("{}-{}-{}", s1, s2, s3);
    // When doing simple addition, the first argument is moved. Does the str reference type not
    // implement the + operator?
    let s = s1 + "-" + &s2 + "-" + &s3;
    // println!("{}", s1); s1 has been moved!
    println!("{}", s2);
    println!("{}", s3);

    // We can't index into a string due to UTF-8 encoding and resulting ambiguity in what is really
    // desired fromt he index
    // Also, the expectation with indexing is O(1) access, whereas thay might not be possible for
    // Strings, due to the UTF-8 encoding of characters.
    let hello = "Здравствуйте";
    // Not allowed:
    // let answer = &hello[0];
    //
    // Instead, rust supports getting a string slice by specifying the byte range
    let s: &str = &hello[..2]; // index out-of-range will not be detected at compile time
    println!("{}", s);

    // In rust, we can think of strings in 3 ways:
    // 1. Sequence of bytes (vec[u8])
    // 2. Unicode scalar values (these are what the'char' type in rust would have)
    // 3. Grapheme Clusters
    // e.g., the string "नमस्ते" can be:
    // 1. [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
    // 2. ['न', 'म', 'स', '्', 'त', 'े'] // Note that the 4th and 6th elements in this array are not
    //    really characters, but instead diatrics which do not make sense in isolation.
    // 3. ["न", "म", "स्", "ते"]
    //
    // Apart from string slicing, rust also provides a way to iterator over the bytes or the
    // characters. Iteration over grapheme clusters is not provided through the standard library.
    //
    let hello = "नमस्ते";
    for b in hello.bytes() {
        println!("{}", b);
    }
    for c in hello.chars() {
        println!("{}", c);
    }
}
