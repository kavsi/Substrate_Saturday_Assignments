//Reference problem solution
fn main() {
    let mut s = String::from("hello, ");

    push_str(&mut s);    // used &mut before s

    println!("Success!");
}

fn push_str(s: &mut String) {
    s.push_str("world")
}
