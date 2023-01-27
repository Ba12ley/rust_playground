fn main() {
    let x :u8 = 6; // u8 means unsigned 32-bit integer, range 0 to 255
    let mut y:i8 = 7; // mut means mutable, i8 means signed 8-bit integer, range -128 to 127
    println!("x = {}, y = {}", x, y);
    y = 8;
    println!("x = {}, y = {}", x, y);

    let floating_point :f32 = 3.14; // f32 means 32-bit floating point number and is default, f64 is 64-bit floating point number
    let t_or_f :bool = true;
    let c :char = 'a'; // char is a single character, single quotes
    let tup : (i32, f64, u8, bool) = (500, 6.4, 1, false); // tuple, can hold different types
    print!("tup = ({}, {}, {}, {})", tup.0, tup.1, tup.2, tup.3);
    let arr : [i32; 5] = [1, 2, 3, 4, 5]; // array, fixed length, all elements must be same type
    println!("arr = {} {}", arr[0], arr[4]); // {:?} is debug print
}
