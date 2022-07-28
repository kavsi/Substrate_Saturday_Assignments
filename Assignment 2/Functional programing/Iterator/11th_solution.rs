// Iterator adaptors problem solution
use std::collections::HashMap;
fn main() {
    let names = ["sunface", "sunfei"];
    let ages = [18, 18];
    let folks: HashMap<_, _> = names.into_iter().zip(ages.into_iter()).collect();  // the blank is filled with 'zip(ages.into_iter())'

    println!("{:?}",folks);
}