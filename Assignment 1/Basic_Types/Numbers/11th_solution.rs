//Computations problem solution
fn main() {
    // Integer addition
    assert!(1u32 + 2 == 3);             // value is 3

    // Integer subtraction
    assert!(1i32 - 2 == -1);            // value is -1
    assert!(1i8 - 2 == -1);             // u8 changed to i8
    
    assert!(3 * 50 == 150);             // value is 150

    assert!(9 / 3 == 3);                // float values changed to integers

    assert!(24 % 5 == 4);               // value is 4
   
    // Short-circuiting boolean logic
    assert!(true && false == false);    // answer is false
    assert!(true || false == true);     // answer is true
    assert!(!true == false);            // answer is false

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}