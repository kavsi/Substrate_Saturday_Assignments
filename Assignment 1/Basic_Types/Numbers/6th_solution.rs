//Integer problem solution
fn main() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1597);   // After the addition the value of v is 1597.

    println!("Success!");
}
