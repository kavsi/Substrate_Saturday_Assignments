// UTF-8 & Indexing problem solution
// FILL in the blanks
fn main() {
    let mut s = String::new();
    s.push_str("hello");  // the solution here is 's.push_str("hello");'

    // some bytes, in a vector
    let v = vec![104, 101, 108, 108, 111];

    // Turn a bytes vector into a String
    // We know these bytes are valid, so we'll use `unwrap()`.
    let s1 = String::from_utf8(v).unwrap();  // the blank is filled with 'String::from_utf8(v).unwrap()'
    
    
    assert_eq!(s, s1);

    println!("Success!")
}