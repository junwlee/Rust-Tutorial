fn main() {
    // 6 core operators
    // <, >, <=, >=, ==,!=
    
    let cond = 10 <= 10.5; // error: mismatched types

    let cond2 = (10 as f32) <= 10.2;
    println!("{}", cond2);

    let mut cond3 = true;
    cond3 = false;
    println!("{}", cond3);

    // Compound conditions
    // logical operators : &&, ||, !

    // Control flow
    let food = "fruits";
    if (food == "pizza") {
        println!("I like pizza");
    } else if food == "fruits" {
        println!("That's sounds healthy!");
    } else {
        println!("I don't like that!");
    }
}
