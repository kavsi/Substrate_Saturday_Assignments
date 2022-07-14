//Reference problem solution
fn main() {
    let mut s = String::from("hello, ");

    let p = &mut s;       // used '&mut s' as s is mutable
    
    p.push_str("world");

    println!("Success!");
}
