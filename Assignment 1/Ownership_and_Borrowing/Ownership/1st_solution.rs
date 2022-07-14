//Ownership problem solution
//1st method:
fn main() {
    let x = String::from("hello, world");
    let y = x;          // x is moved to y. so x will no longer be valid
    println!("{}",y);   // removed x from printing
}

//2nd method:
fn main() {
    let x = String::from("hello, world");
    let y = x.clone();  // used .clone() to not drop x
    println!("{},{}",x,y);
}

//3rd method:
fn main() {
    let x = "hello, world";  //removed String::from from the expression
    let y = x;
    println!("{},{}",x,y);
}