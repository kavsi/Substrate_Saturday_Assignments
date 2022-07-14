//Ownership problem solution
fn main() {
    let s = give_ownership();
    println!("{}", s);
}

fn give_ownership() -> String {
    let s = String::from("hello, world");
    // Convert String to Vec
    let _s = s.as_bytes();       // converted into_bytes to as_bytes 
    s
}
