// Implement From for custom types problem solution
// From is now included in `std::prelude`, so there is no need to introduce it into the current scope
// use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    // IMPLEMENT `from` method
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let num = Number::from(30);  // the blank is filled with 'Number::from'
    assert_eq!(num.value, 30);

    let num: Number = 30.into();  // the blank is filled with '30.into()'
    assert_eq!(num.value, 30);

    println!("Success!")
}