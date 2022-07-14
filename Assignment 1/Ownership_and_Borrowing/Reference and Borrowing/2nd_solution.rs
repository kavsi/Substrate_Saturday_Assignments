//Reference problem solution
fn main() {
    let x = 5;
    let y = &x;

    // Modify this line only
    assert_eq!(5, *y);   // Used dereference operator '*' to point the value of variable 'x'

    println!("Success!");
}
