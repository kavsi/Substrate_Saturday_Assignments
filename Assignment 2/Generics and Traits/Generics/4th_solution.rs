// Struct and impl problem solution
// modify this struct to make the code work
struct Point<T, U> {  // here 'U' is added.
    x: T,
    y: U,             // here 'T' is changed to 'U'
}

fn main() {
    // DON'T modify here
    let p = Point{x: 5, y : "hello".to_string()};
}