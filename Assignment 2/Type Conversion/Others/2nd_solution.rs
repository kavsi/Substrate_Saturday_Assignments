// Parse a String problem solution
// To use `from_str` method, you needs to introduce this trait into the current scope.
use std::str::FromStr;
fn main() {
    let parsed: i32 = "5".parse().unwrap();     // the blank is filled with 'parse()'
    let turbo_parsed = "10".parse::<i32>().unwrap();   // the blank is filled with 'parse::<i32>()'
    let from_str = i32::from_str("20").unwrap();   // the blank is filled with 'i32::from_str("20")'
    let sum = parsed + turbo_parsed + from_str;
    assert_eq!(sum, 35);

    println!("Success!");
}