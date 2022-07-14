//String problem solution
fn main() {
    let s = String::from("I like dogs");
    // Allocate new memory and store the modified string there
    let s1 = s.replace("dogs", "cats");   // the 'replace' can be used to replace substring
 
    assert_eq!(s1, "I like cats")
 }