// result and ? problem solution
use std::num::ParseIntError;

fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {  // the blank is filled with 'Result<i32, ParseIntError>'
    let n1 = n1_str.parse::<i32>();
    let n2 = n2_str.parse::<i32>();
    Ok(n1.unwrap() * n2.unwrap())
}

fn main() {
    let result = multiply("10", "2");
    assert_eq!(result, Ok(20));  // the blank is filled with 'Ok(20)'

    let result = multiply("4", "2");
    assert_eq!(result.unwrap(), 8);  // the blank is filled with 'unwrap'

    println!("Success!");
}