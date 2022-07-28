// Method problem solution
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // In area, we use &self instead of rectangle: &Rectangle.
    // The &self is actually short for self: &Self.
    // We’ve chosen &self because we don’t want to take ownership, and we just want to read the data in the struct, not write to it.
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    assert_eq!(rect1.area(), 1500);
    
    println!("Success!");
}