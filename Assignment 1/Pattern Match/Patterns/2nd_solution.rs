//Patterns problem solution
struct Point {
    x: i32,
    y: i32,
}

fn main() {

    let p = Point { x: 2, y: 20 }; // the x can be [0, 5] and y can be 10 20 or 30

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        // second arm
        Point { x: 0..=5, y: y@ (10 | 20 | 30) } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}