//Slice problem solution
fn main() {
    let arr = [1, 2, 3];
    let s1: &[i32] = &arr[0..2];      // the '&' operator is used before [132] and arr[0..2]

    let s2: &str = "hello, world";    // the '&' operator is used before str and 'as str' is removed

    println!("Success!");
}