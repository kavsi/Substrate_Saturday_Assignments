//Shadowing problem solution
fn main() {
    let mut x: i32 = 1;
    x = 7;
    // Shadowing and re-binding
    // The line (let x = x;) is removed
    x += 3;

    let y = 4;
    // Shadowing
    let y = "I can also be bound to text!"; 

    println!("Success!");
}
