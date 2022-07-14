//&str and String problem solution
//1st method:
fn main() {
    let s = "hello, world".to_string();  // '.to_string()' is used for the input of string
    greetings(s)
}

fn greetings(s: String) {
    println!("{}",s)
}

//2nd method:
fn main() {
    let s = String::from("hello, world"); // 'String::from()' is also used for the input of string
    greetings(s)
}

fn greetings(s: String) {
    println!("{}",s)
}
