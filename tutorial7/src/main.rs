use std::io;

fn main() {
    introduce();

    // Statements and expressions
    let number = {
        let x = 3;
        x * 2 // x * 2; <-- error
    };
    println!("The number is {}", number);

    let result = add_numbers(10, 20);
    println!("The result is {}", result);
}

fn introduce() {
    let mut name = String::new();
    println!("Enter your name: ");
    io::stdin().read_line(&mut name).expect("failed to read line");
    println!("Hello, my name is {}!", name.trim());
}

fn add_numbers(x: i32, y: i32) -> i32 {
    let result = x + y;
    if result > 20 {
        return result - 5;
    }
    result
}