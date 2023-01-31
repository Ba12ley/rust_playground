extern crate regex;
use regex::Regex;

pub fn regex_start() {
    let re = Regex::new(r"\w{5}").unwrap(); // \w is a word character, {5} is the number of characters
    let word = "Words";
    println!("Does {} match the regex? {}", word, re.is_match(word));
}
