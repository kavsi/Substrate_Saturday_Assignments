//String slicesproblem solution
fn main() {
    let s = String::from("hello");

    let slice1 = &s[0..2];
    let slice2 = &s[..2];  // the blank is filled with '&s[..2]'

    assert_eq!(slice1, slice2);
    
    println!("Success!");
}