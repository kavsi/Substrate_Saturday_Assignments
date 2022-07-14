//String problem solution
fn main() {
    let mut s = String::new();    // 'string::new()' lets the variable 's' to create new empty string 
    s.push_str("hello, world");
    s.push('!');

    assert_eq!(s, "hello, world!");

    println!("Success!");
}