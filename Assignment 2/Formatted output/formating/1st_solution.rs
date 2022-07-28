// Positional arguments problem solution
fn main() {
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob"); // => Alice, this is Bob. Bob, this is Alice
    assert_eq!(format!("{1}{0}", 1, 2), "21");       // the blank is filled with value "21"
    assert_eq!(format!("{1}{}{0}{}", 1, 2), "2112");   // the blank is filled with "{1}{}{0}{}"
    println!("Success!");
}