//String problem solution
fn main() {
    let mut s = String::from("hello");
     s.push(',');
     s.push_str(" world");    // 'push' changed to 'push_str'
     s += "!";                // '.to_string()' is removed
 
     println!("{}", s)
 }