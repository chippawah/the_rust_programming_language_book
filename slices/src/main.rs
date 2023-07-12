fn main() {
    let mut my_string = String::from("Hello World");
    let end_of_first_word = first_word(&my_string);
    my_string.clear();
    println!("Index of the end of the first word even though my_string was cleared: {end_of_first_word}");
    // The above doesn't really help us because the index would no longer be valid. 
    // Thankfully Rust has a String slice type that's like a reference but to only part of a string
    let mut my_string = String::from("Hello World");
    let hello = &my_string[..5]; // note the range syntax here
    let world = &my_string[6..];
    println!("The hello variable is equal to the referenced string slice (&str) '{hello}'");
    println!("The world variable is equal to '{world}'");
    // Slices make it much easier for us to refer to a part of a string like the above examples
    // Now we can write a function that returns the actual word but returning a slice of the word.
    let word = better_first_word(&my_string);
    // The next line shows ones that would throw a compiler error because my_string is borrowed immutably and the .clear() method mutates the string.
    // my_string.clear();
    println!("The first word in my_string is: {word}");
    // .clear() can be called here after the last use of word because it's a reference and ends the last time its used
    my_string.clear();
    // my_string_literal variable below is a string slice pointing to the string literal in the binary
    let my_string_literal = "Hello World";
    let word = rusty_first_word(my_string_literal);
    println!("The string slice of the string slice of the string literal from rusty_first_word: {word}");
    let my_string = String::from("Hello World");
    let word = rusty_first_word(&my_string);
    println!("The string slice of the String from rusty_first_word: {word}");
}

// The non idiomatic way of getting the first word in a String by returning the the index at the end of the first word
// As we'll see this can cause an issue later if the underlying String is modified
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return i;
        }
    }
    return s.len();
}

// This will instead return a string slice that represents the first word.
fn better_first_word(s: &String) -> &str {
    for (i, &letter) in s.as_bytes().iter().enumerate() {
        if letter == b' ' {
            return &s[..i]
        }
    }
    return &s[..];
}

fn rusty_first_word(s: &str) -> &str {
    for (i, &letter) in s.as_bytes().iter().enumerate() {
        if letter == b' ' {
            return &s[..i]
        }
    }
    return &s[..];
}