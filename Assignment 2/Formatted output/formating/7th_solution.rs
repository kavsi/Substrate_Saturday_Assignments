// precision problem solution
fn main() {
    let s = "Hello, world!";

    println!("{0:.5}", s); // => Hello

    assert_eq!(format!("Hello {1:.0$}!", 3, "abcdefg"), "Hello abc!");  // the blank is filled with '{1:.0$}'

    println!("Success!")
}