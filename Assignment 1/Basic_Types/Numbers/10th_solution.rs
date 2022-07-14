//Floating-Point problem solution
use std::ops::{Range, RangeInclusive};
fn main() {
    assert_eq!((1..5), Range{ start: 1, end: 5 });   // Here 5 is not included
    assert_eq!((1..=5), RangeInclusive::new(1, 5));  // Here 5 is included

    println!("Success!");
}