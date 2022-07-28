// Exercises problem solution
// Fill in the blanks to make it work.
fn print_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {  // the blanks are filled with 'T: std::fmt::Debug, const N: usize' and 'arr: [T; N]'
    println!("{:?}", arr);
}
fn main() {
    let arr = [1, 2, 3];
    print_array(arr);

    let arr = ["hello", "world"];
    print_array(arr);
}