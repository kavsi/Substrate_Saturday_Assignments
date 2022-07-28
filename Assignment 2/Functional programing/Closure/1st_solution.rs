// Capturing problem solution
fn main() {
    let color = String::from("green");

    let print = || println!("`color`: {}", color);

    print();
    print();

    // `color` can be borrowed immutably again, because the closure only holds
    // an immutable reference to `color`. 
    // this line is removed 'let _reborrow = &color;'
    
    println!("{}",color);
}