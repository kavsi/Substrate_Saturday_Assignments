//Functions problem solution
//1st method:
fn main() {
    never_return();      // removed println!("Failed!") statement
}

fn never_return() -> ! {
    // Implement this function, don't modify the fn signatures
    panic!("i return nothing")   // implementing panic! for creating error situation
}

//2nd method:
fn main() {
    never_return();
}

use std::thread;
use std::time;

fn never_return() -> ! {
    // implement this function, don't modify fn signatures
    loop {
        println!("I return nothing");                // implementing panic! for creating error situation
        thread::sleep(time::Duration::from_secs(1))  // sleeping for 1 second to avoid exhausting the cpu resoucre
    }
}