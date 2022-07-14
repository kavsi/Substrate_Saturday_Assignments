//Str and &str problem solution
//1st method:
fn main() {
    let s: Box<&str> = "hello, world".into();
    greetings(&s)    // '&' can be used to convert 'Box<str>' to '&str'
}

fn greetings(s: &str) {
    println!("{}", s);
}

//2nd method:
fn main() {
    let s: Box<&str> = "hello, world".into();
    greetings(*s)    // 's' is replaced with '*s'
}

fn greetings(s: &str) {
    println!("{}", s);
}