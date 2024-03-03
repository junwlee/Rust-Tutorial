use std::io;

fn main() {
    println!("Input here: ");

    let mut input = String::new();
    io::stdin().read_line(& mut input).expect("failed to read line");
    // stdin().read_line(&mut input) function waits
    // until the user presses the enter key to finish their input
    // and then it stores all the entered content (including the enter key) in 'input'
    println!("Input here: {}", input.trim());
}
