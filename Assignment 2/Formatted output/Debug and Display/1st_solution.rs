// Debug problem solution
#[derive(Debug)]   // All types can derive the std::fmt::Debug implementation and we can derive the automatic implementations provided by Rust using this line
struct Structure(i32);

fn main() {
    // Types in std and Rust have implemented the fmt::Debug trait
    println!("{:?} months in a year.", 12);  // {:?} must be used to print out the type which has implemented the Debug trait.

    println!("Now {:?} will print!", Structure(3));  // {:?} must be used to print out the type which has implemented the Debug trait.
}