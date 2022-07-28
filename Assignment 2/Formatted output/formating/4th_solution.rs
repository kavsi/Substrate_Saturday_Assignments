// Padding with string problem solution
fn main() {
    // left align
    println!("Hello {:<5}!", "x"); // => Hello x    !
    // right align
    assert_eq!(format!("Hello {:>5}!", "x"), "Hello     x!");  // the blank is filled with '{:>5}'
    // center align
    assert_eq!(format!("Hello {:^5}!", "x"), "Hello   x  !");  // the blank is filled with '{:^5}'

    // left align, pad with '&'
    assert_eq!(format!("Hello {:&<5}!", "x"), "Hello x&&&&!"); // the blank is filled with 'x&&&&!'

    println!("Success!")
}