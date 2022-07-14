//Ownership problem solution
fn main() {
    let x = (1, 2, (), "hello");
    let y = x;                     // used copy instead of cloning
    println!("{:?}, {:?}", x, y);
}