use std::io;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}
const MAX_NUMBER:u16 = 360;

fn main() {
    let x: u8 = 6; // u8 means unsigned 32-bit integer, range 0 to 255
    let mut y: i8 = 7; // mut means mutable, i8 means signed 8-bit integer, range -128 to 127
    println!("x = {}, y = {}", x, y);
    y = 8;
    println!("x = {}, y = {}", x, y);

    let floating_point: f32 = 3.14; // f32 means 32-bit floating point number, f64 is 64-bit floating point number and the default
    let t_or_f: bool = true;
    let c: char = 'a'; // char is a single character, single quotes
    let tup: (i32, f64, u8, bool) = (500, 6.4, 1, false); // tuple, can hold different types
    print!("tup = ({}, {}, {}, {})", tup.0, tup.1, tup.2, tup.3);
    let nest_tup: (i32, f64, u8, (bool, char)) = (500, 6.4, 1, (false, 'a'));
    println!("nested tuple {}", nest_tup.3.1); // nested tuple access
    let arr: [i32; 5] = [1, 2, 3, 4, 5]; // array, fixed length, all elements must be same type
    println!("arr = {} {}", arr[0], arr[4]); // {:?} is debug print

    println!("Enter some Input");
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line"); // & means reference, mut means mutable

    println!("You entered: {}", user_input);

    let a: i64 = 25;
    let b: i64 = 5;
    let d = 10.0_f64; // another way to declare a float type
    let e = 256 as i64; // another way to declare a type

    //let z = x + y; // z is inferred to be i8, the src types are different and cannot be added
    let z = x + y as u8; // z is inferred to be u8, y is cast to u8
    let c = a + b; // c is inferred to be i64
    println!("z = {}, c = {}", z, c);

    let mut convert_to_int = String::new();
    io::stdin()
        .read_line(&mut convert_to_int)
        .expect("Failed to read line");
    let int_from_string = convert_to_int
        .trim()
        .parse::<i32>()
        .expect("Failed to convert to int");
    println!("You entered: {}", int_from_string);

    let condition = 1 > 2;
    println!("condition = {}", condition);

    let item = "banana";
    if item == "banana" {
        println!("I like bananas");
    } else if item == "apple" {
        println!("I like apples");
    } else {
        println!("I don't like {}", item);
    }

    first_function(3, 8); // call a function
    counter(20);
    even_counter(40);
    using_for();
    using_vector();
    using_enum();
}

fn first_function(x: i32, y: i32) -> i32 {
    println!("The sum of x and y is {}", x + y);
    x + y // return value as an expression, no semicolon.  Or use return keyword (return x + y;) as a statement
}

fn counter(limit: i32) -> i32 {
    let mut count = 0;
    loop {
        if count >= limit {
            break;
        } else {
            count += 1;
            println!("The count is {}", count);
        }
    }
    count
}

fn even_counter(limit: i32) -> i32 {
    let mut count = 0;
    while count < limit {
        if count % 2 == 0 {
            println!("The count is {}", count);
        }
        count += 1;
    }
    count
}

fn fives_counter(limit: i32) -> i32 {
    let mut count = 0;
    while count < limit {
        if count % 5 == 0 {
            println!("The count is {}", count);
        }
        count += 1;
    }
    count
}
//for loops must use an iterator, the range can be declared as below or with a variable let x = 1..100;
fn using_for() {
    for i in 1..100 {
        println!("{}", i);
    }
}

fn using_vector() {
    let mut cars:Vec<&str> = vec!["Ford", "Chevy", "Toyota"];
    cars.push("Honda");
    for (index, car) in cars.iter().enumerate() { // iter() is an iterator and will prevent ownership moving into the for loop.  cars is still available after the loop
        println!("Index {} for {}", index, car);
    }
    println!("{}", cars[0]) // cars is still available after the for loop
}

fn using_enum() {
    let direction = Direction::Up; // :: is the scope operator
    match direction {
        Direction::Up => println!("Going up"),
        Direction::Down => println!("Going down"),
        Direction::Left => println!("Going left"),
        Direction::Right => println!("Going right"),
    }
}