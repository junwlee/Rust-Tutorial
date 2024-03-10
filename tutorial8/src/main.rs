fn add(x: i32, y:i32) -> i32 {
    x + y // top of the stack and we retrieve the x and y
    
    // Once this scope is finished,
    // we're going to pop off x, y, and return the result.
}

fn main() {
    let x = 2; // stack
    let y = x; // stack

    add(x, y);

    //Heap
    let string1 = String::from("hello"); // 'from' function takes a string literal ("hello") as an argument and returns a String instance.
    // the name of the variable string1 will be stored in the stack
    // string1 are going to have a reference(pointer) to the heap
    // we need to find the address or the location in the heap where the string1 is stored

    // String instances are allocated on the heap. 
    // In theory, a String instance has the potential to be modified at runtime. 
    // However, since the variable string1 is declared as immutable, 
    // directly changing the string stored within this instance or altering its size is not permitted.
}