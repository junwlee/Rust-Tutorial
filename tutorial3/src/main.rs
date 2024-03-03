fn main() {

// scalar data type
    let x = -2;
    // let x:i32 = -2;
    // i8, i16, i32, i64, i128

    let y = 4;
    // u8, u16, u32, u64, u128
    println!("{}, {}", x, y);

    // bool, char


// compound data type
    let mut tup1:(i16, bool, char) = (1, false, 'a');
    tup1.0 = 5; 
    // tup1.1 = 5 <-- mismatched types
    // tup1.3 = 10 <-- unknown field

    let tup2 = (true, 'b', 6);
    println!("{}, {}", tup1.0, tup2.1);

    let arr1 = [1, 2, 3, 4, 5];
    let mut arr2: [i32; 5] = [1, 2, 3, 4, 5];
    arr2[3] = 10;
    println!("{}, {}", arr1[arr1.len() -1], arr2[3]); // 5, 10
}