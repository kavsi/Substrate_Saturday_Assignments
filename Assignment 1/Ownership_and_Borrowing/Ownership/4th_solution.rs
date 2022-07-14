//Ownership problem solution
fn main() {
    let s = String::from("hello, world");

    print_str(s.clone());   // cloning the string s

    println!("{}", s);
}

fn print_str(s: String)  {
    println!("{}",s)
}