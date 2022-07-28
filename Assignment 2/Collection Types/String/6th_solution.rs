// Representation problem solution
// modify the code below to print out: 
// 25
// Here, there’s no need to allocate more memory inside the loop.
fn main() {
    let mut s = String::with_capacity(25);  // instead of 'new' we used 'with_capacity'

    println!("{}", s.capacity());

    for _ in 0..2 {
        s.push_str("hello");
        println!("{}", s.capacity());
    }

    println!("Success!")
}