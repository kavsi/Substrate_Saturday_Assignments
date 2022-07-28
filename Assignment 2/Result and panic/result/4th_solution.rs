// map & and_then problem solution
// 1st method:
use std::num::ParseIntError;

fn add_two(n_str: &str) -> Result<i32, ParseIntError> {
   n_str.parse::<i32>().map(|num| num +2)  // the blank is filled with 'map(|num| num +2)'
}

fn main() {
    assert_eq!(add_two("4").unwrap(), 6);

    println!("Success!");
}

// 2nd method:
use std::num::ParseIntError;

fn add_two(n_str: &str) -> Result<i32, ParseIntError> {
   n_str.parse::<i32>().and_then(|num| Ok(num +2))   // the blank is filled with 'and_then(|num| Ok(num +2))'
}

fn main() {
    assert_eq!(add_two("4").unwrap(), 6);

    println!("Success!");
}