//Statements and Expressions Exercises solution
fn main() {
    let s = sum(1 , 2);
    assert_eq!(s, 3);

    println!("Success!");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y        // Semicolon is removed to return the value x+y
}

