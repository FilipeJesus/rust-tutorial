fn main() {
    // When talking about strings in rust, you might be talking about
    // either the String type or the string slice &str. This page
    // will be about the String type

    // In Rust strings are UTF-8 encoded, so you can include any
    // unicode character in a string

    // Many of the same operations that work on vectors also work on
    // strings. This is because strings are implemented as a vector of
    // bytes

    // Creating new string
    let mut s = String::new();

    // But if you have a string literal, you can use the to_string
    // method to create a string
    let data = "initial contents";
    let s = data.to_string();

    // You can also use the String::from function
    let mut s = String::from("foo");

    // You can append to a string using a couple of methods.

    // The push_str method takes a string slice
    s.push_str("bar");
    println!("s is {s}");

    // The push method appends a single character
    s.push('.');
    println!("s is {s}");

    // You can also concatenate strings using the + operator
    // This takes ownership of s, so they can no longer be used
    let s2 = s + "baz";
    println!("s2 is {s2}");

    // The format! macro can also be used to concatenate strings
    let s3 = String::from("foo")
    let s = format!("{s2}-{s3}");
    println!("s is {s}");

    // You can't index into a string in rust, because strings are
    // stored as a vector of bytes, and indexing into a string would
    // return a byte, not a character. You can use the chars method to
    // iterate over the characters of a string
    for c in "hello".chars() {
        println!("{c}");
    }

    // You can also loop over each raw byte
    for b in "hello".bytes() {
        println!("{b}");
    }

}
