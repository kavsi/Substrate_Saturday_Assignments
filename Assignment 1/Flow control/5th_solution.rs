//For problem solution
fn main() {
    let a = [4, 3, 2, 1];

    // iterate the indexing and value in 'a'
    for (i, v) in a.iter().enumerate() {       // the blank  is filled with iter().enumerate()
        println!("The {}th element is {}", i + 1, v);
    }
}