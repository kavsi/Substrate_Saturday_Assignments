//Tuple problem solution
fn main() {
    let tup = (1, 6.4, "hello");

    let (x,z,y) = tup;  // the blank is filled with (x,z,y)

    assert_eq!(x, 1);
    assert_eq!(y, "hello");
    assert_eq!(z, 6.4);

    println!("Success!");
}
