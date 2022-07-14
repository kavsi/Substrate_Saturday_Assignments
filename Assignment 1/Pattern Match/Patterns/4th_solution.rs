//Patterns problem solution
fn main() {
    let num = Some(4);
    let split = 5;
    match num {
        Some(x) if x < split => assert!(x < split),  // the condition is x is less than split
        Some(x) => assert!(x >= split),
        None => (),
    }
    
    println!("Success!");
}