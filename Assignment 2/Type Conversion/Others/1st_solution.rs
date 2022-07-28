// Convert any type to String problem solution
use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {  // implemented 'fmt' method
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "The point is ({}, {})", self.x, self.y)
    }
}

fn main() {
    let origin = Point { x: 0, y: 0 };
    assert_eq!(origin.to_string(), "The point is (0, 0)");    // the blank is filled with 'to_string()'
    assert_eq!(format!("{}", origin), "The point is (0, 0)");     // the blank is filled with '("{}", origin)'

    println!("Success!");
}