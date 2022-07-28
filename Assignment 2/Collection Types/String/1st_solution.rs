// Basic operations problem solution
// 1st method:
fn main() {
    let mut s: String = String::from("hello, ");  // used 'String::from' function to insert into 's'
    s.push_str("world");   // the 'to_string()' is removed
    s.push('!');   // the blank is filled with '!'

    move_ownership(s.clone()); // ownership is moved to 's.clone()'

    assert_eq!(s, "hello, world!");

    println!("Success!")
}

fn move_ownership(s: String) {
    println!("ownership of \"{}\" is moved here!", s)
}

// 2nd solution:
fn main() {
    let mut s: String = String::from("hello, ");  // used 'String::from' function to insert into 's'
    s.push_str("world");  // the 'to_string()' is removed
    s.push('!');   // the blank is filled with '!'

    borrow_string(&s);  // instead 'move_ownership(s)' we used 'borrow_string(&s)' function

    assert_eq!(s, "hello, world!");

    println!("Success!")
}

fn borrow_string(s: &str) {
    println!("ownership of \"{}\" is moved here!", s)
}