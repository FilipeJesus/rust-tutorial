use std::collections::HashMap;

fn main() {
    // Hash maps are made using HashMap<k, v>
    // They are like dictionaries in python

    let mut scores = HashMap::new();

    // You can insert values into a hash map using the insert method
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // You can use get to get the value of a key
    // This returns an Option<&v>. If there is no value, it returns None
    // You can use the unwrap method to get the value, or a default value
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    // You can loop through the keys and values of the hash map
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // When the value is of a type which does not implement Copy.
    // Hash maps take ownership of the values they store, so if you store
    // a String variable you can't use it after. But i32 values are copied
    // so you can still use them

    // Overwriting a value
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    // Only insert if the key is not already present, this is done
    // using the entry method along with the or_insert method which
    // is a method of the Entry enum
    scores.entry(String::from("Yellow")).or_insert(30);
    scores.entry(String::from("Green")).or_insert(30);
    println!("{:?}", scores);

    // entry combined with the or_insert method returns a mutable reference
    // to the value, so you can modify it
    let text = "hello world wonderful world";
    let mut map = HashMap::new();\
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

}
