use std::collections::HashMap;

pub fn using_hash_maps() {
    let mut scores = HashMap::new(); //HashMap is a collection of key-value pairs, like a dictionary in Python

    scores.insert(String::from("Blue"), 10);// insert takes ownership of the key and value.  key must be a string
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Red"), 100);

    println!("{:?}", scores);

    match scores.get("Yellow") {
        Some(score) => println!("The score is {}", score),  //Some is an enum that can be Some or None
        None => println!("No score found"),
    }

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    scores.remove("Blue");

    println!("{}", scores.contains_key("Blue")); // contains_key searches for a key and returns a boolean
}

