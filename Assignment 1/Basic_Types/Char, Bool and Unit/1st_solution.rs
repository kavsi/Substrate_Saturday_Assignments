//Char problem solution
use std::mem::size_of_val;
fn main() {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1),4);  // the value is changed from 1 to 4 because size is 4

    let c2 = '中';
    assert_eq!(size_of_val(&c2),4);  // the value is changed from 3 to 4 because size is 4

    println!("Success!");
} 