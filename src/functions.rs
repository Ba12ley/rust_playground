use std::fs::File;
use std::io::prelude::*;

pub fn read_file(){
    let mut f = File::open("test.txt")
        .expect("File not found"); //expect is a macro that prints the error message if the result is Err
    let mut file_contents = String::new();
    f.read_to_string(&mut file_contents)
        .expect("Something went wrong reading the file");

    println!("File contents: {}", file_contents);
}

pub fn write_file(){
    let mut f = File::create("output.txt")
        .expect("File not found");
    f.write_all(b"Did this overwrite or append?")//b is a byte literal
        .expect("Something went wrong writing the file");
}