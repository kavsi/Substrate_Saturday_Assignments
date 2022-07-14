//Binding and mutability problem solution
fn main() {
    let mut x =  1;         //adding "mut" keyword in front of x to make it mutable.
    x += 2; 
    
    assert_eq!(x, 3);
    println!("Success!");
}

