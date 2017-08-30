// Question: Convert strings to Pig Latin, where the first consonant of each
// word is moved to the end of the word with an added “ay”, so “first” becomes
// “irst-fay”. Words that start with a vowel get “hay” added to the end instead
// (“apple” becomes “apple-hay”). Remember about UTF-8 encoding!

pub fn convert_to_piglatin(input: &str) -> String {
    use std::collections::HashSet;
    // This is magic to create a hashset from a vector.
    let vowels: HashSet<char> = ['a', 'e', 'i', 'o', 'u'].iter().cloned().collect();
    // We'll collect all the modified words in this vector of strings.
    let mut result: Vec<String> = Vec::new();
    for word in input.split_whitespace() {
        // Question for later: Can we improve this?
        if let Some(c) = word.chars().next() {
            let mut modified_word = String::new();
            if vowels.contains(&c) {
                append_hay(word, &mut modified_word);
            } else {
                obfuscate(word, c, &mut modified_word);
            }
            result.push(modified_word);
        } else {
            panic!("We should never reach here");
        }
    }
    result.join(" ")
}

fn append_hay(word: &str, result: &mut String) {
    result.push_str(word);
    result.push_str("-hay");
}

fn obfuscate(word: &str, c: char, result: &mut String) {
    result.push_str(&word[1..]);
    result.push('-');
    result.push(c);
    result.push_str("ay");
}
