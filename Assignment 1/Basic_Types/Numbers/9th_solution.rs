//Floating-Point problem solution
fn main() {
    let mut sum = 0;
    for i in -3..2 {       // Here 2 is not included.
        sum += i
    }

    assert!(sum == -5);    // Hence sum is changed from -3 to -5

    for c in 'a'..='z' {
        println!("{}",c as u8);  // u8 is added to change alphabets to numericals
    }
}