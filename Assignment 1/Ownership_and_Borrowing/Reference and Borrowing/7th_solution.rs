//Borrowing rules problem solution
fn main() {
    let s = String::from("hello");

    let r1 = &s;     // Here doesn't need to be mutable
    let r2 = &s;

    println!("{}, {}", r1, r2);

    println!("Success!");
}
