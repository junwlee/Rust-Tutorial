fn main() {
    let mut x = 5;
    println!("x is {}", x);
    
    {
        println!("x is {}", x);
        let x = x + 1;
        println!("x is {}", x);
    }
    
    println!("x is {}", x);
    x = 1;
    println!("x is {}", x);


    // Constant
    const SECONDS_IN_MINUTE: u32 = 60;
    println!("{}", SECONDS_IN_MINUTE);
}
