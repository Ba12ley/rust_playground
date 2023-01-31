extern crate regex;
use regex::Regex;

pub fn regex_start() {
    let re = Regex::new(r"\w{5}").unwrap(); // \w is a word character, {5} is the number of characters
    let word = "Words";
    match re.captures(word) {
        Some(caps) => println!("Matched: {}", &caps[0]), // &caps[0] is the first capture group
        None => println!("No match"),
    }

}
