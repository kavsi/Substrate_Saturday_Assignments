// Multiple impl blocks problem solution
struct Rectangle {
    width: u32,
    height: u32,
}

// rewrite Rectangle to use multiple `impl` blocks
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {                                         // used 'impl' as each struct is allowed to have multiple impl blocks.
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    println!("Success!");
}