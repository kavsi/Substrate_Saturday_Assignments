//Match problem solution
fn main() {
    let boolean = true;

    let binary = match boolean {  
        true => 1,      // boolean = true => binary = 1
        false => 0      // boolean = false =>  binary = 0
    };

    assert_eq!(binary, 1);
    
    println!("Success!");
}