// Struct and impl problem solution
// Implemented struct Point.
struct Point<T> {  
    x: T,      // here 'x' and 'y' are attributes.
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}