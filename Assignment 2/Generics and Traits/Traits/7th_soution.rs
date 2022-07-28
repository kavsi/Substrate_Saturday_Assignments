// Trait bound problem solution
fn main() {
    assert_eq!(sum(1, 2), 3);
    assert_eq!(sum(1.0, 2.0), 3.0);  // this line is added
}

fn sum<T: std::ops::Add<Output = T>>(x: T, y: T) -> T {  // we replaced 'T' with 'T: std::ops::Add<Output = T>'
    x + y
}