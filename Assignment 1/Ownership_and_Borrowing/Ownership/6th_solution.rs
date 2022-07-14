//Mutability problem solution
fn main() {
    let s = String::from("hello, ");
    
    let mut s1 = s;         // adding mut keyword to s1 to make it mutable 

    s1.push_str("world");

    println!("Success!");
}
