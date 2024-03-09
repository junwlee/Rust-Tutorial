use std::io;

fn main() {
    // let a: i8 = 10; // -128 ~ 127
    // let b: u8 = 25; // 0 ~ 255
    
    // let c = a + b; // mismatched types

    
    // let x: f32 = 10; // 10.0
    // let y: f64 = 20; // 20.0

    // let z = x + y; // mismatched types
    
    
    // let d = 200 as u8;
    // let e = 10 as u8;

    // let f = d / e;

    // let g = 255.0_f64;
    // let h = 10.0_f64;

    // let i = g * h;

    // let k:i16 = 155;
    // let l:i16 = 10;

    // let m = k % l;


    // let n = (i32::MAX as i64) + 1; // i32::MAX = 2147483647
    // let o:i32 = 10;

    // let p = (n as i32) / o;
    // // Since the value is just one unit beyond i32::MAX, it wraps to i32::MIN (which is -2,147,483,648) 
    // // and then gets incremented by 1 due to the overflow wrap, resulting in -2,147,483,647.

    // Converting String to Integer
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading");

    let int_input:i64 = input.trim().parse().unwrap();
    // unwrap() : 
    // Return the Ok value (the parsed integer), allowing the program to proceed with int_input as an i64 type.
    // Cause the program to panic and terminate if the result is Err, which would indicate that the parsing failed (e.g., because the input was not a valid integer).

    println!("{}", int_input + 3);

}