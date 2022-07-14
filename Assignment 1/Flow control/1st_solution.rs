//If/else problem solution
fn main() {
    let n = 5;

    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {                    // the blank is filled with else if
        println!("{} is positive", n);
    } else {                             // the blank is filled with else
        println!("{} is zero", n);
    }
}