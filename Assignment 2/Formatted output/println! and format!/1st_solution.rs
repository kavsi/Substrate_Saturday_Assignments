// format! problem solution
fn main() {
    let s1 = "hello";
    /* Fill in the blank */
    // format! is used write formatted text to [String][string]
    let s = format!("{}, world!", s1);  // used '("{}, world!", s1)' to fill in the blank.
    assert_eq!(s, "hello, world!");
}