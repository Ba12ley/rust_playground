
use std::io;

fn main() {
    let x :u8 = 6; // u8 means unsigned 32-bit integer, range 0 to 255
    let mut y:i8 = 7; // mut means mutable, i8 means signed 8-bit integer, range -128 to 127
    println!("x = {}, y = {}", x, y);
    y = 8;
    println!("x = {}, y = {}", x, y);

    let floating_point :f32 = 3.14; // f32 means 32-bit floating point number, f64 is 64-bit floating point number and the default
    let t_or_f :bool = true;
    let c :char = 'a'; // char is a single character, single quotes
    let tup : (i32, f64, u8, bool) = (500, 6.4, 1, false); // tuple, can hold different types
    print!("tup = ({}, {}, {}, {})", tup.0, tup.1, tup.2, tup.3);
    let arr : [i32; 5] = [1, 2, 3, 4, 5]; // array, fixed length, all elements must be same type
    println!("arr = {} {}", arr[0], arr[4]); // {:?} is debug print

    println!("Enter some Input");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read line"); // & means reference, mut means mutable

    println!("You entered: {}", user_input);

    let a:i64 = 25;
    let b:i64 = 5;
    let d = 10.0_f64; // another way to declare a float type
    let e = 256 as i64; // another way to declare a type

    //let z = x + y; // z is inferred to be i8, the src types are different and cannot be added
    let z = x + y as u8; // z is inferred to be u8, y is cast to u8
    let c = a + b; // c is inferred to be i64
    println!("z = {}, c = {}", z, c);

    let mut convert_to_int = String::new();
    io::stdin().read_line(&mut convert_to_int).expect("Failed to read line");
    let int_from_string = convert_to_int.trim().parse::<i32>().expect("Failed to convert to int");
    println!("You entered: {}", int_from_string);
}
