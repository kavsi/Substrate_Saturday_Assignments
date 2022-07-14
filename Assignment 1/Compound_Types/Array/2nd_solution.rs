//Array problem solution
fn main() {
    // We can ignore parts of the array type or even the whole type, let the compiler infer it for us
    let arr0 = [1, 2, 3];
    let arr: [_; 3] = ['a', 'b', 'c'];

    // Arrays are stack allocated, `std::mem::size_of_val` returns the bytes which an array occupies
    assert!(std::mem::size_of_val(&arr) == 12);  // A char takes 4 bytes in Rust: Unicode char and there are 3 characters here

    println!("Success!");
}
