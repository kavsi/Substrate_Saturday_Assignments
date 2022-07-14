//&str and String problem solution
//1st method:
fn main() {
    let s =  "hello, world".to_string();
    let s1: &str = &s;

    println!("Success!");
}

//2nd method:
fn main() {
    let s =  "hello, world";
    let s1: &str = &s;

    println!("Success!");
}
