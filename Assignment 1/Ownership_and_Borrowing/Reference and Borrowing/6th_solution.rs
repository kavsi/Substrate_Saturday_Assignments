//Reference problem solution
fn main() {
    let c = '中';

    let r1 = &c;

    let ref r2 = c;    // used 'ref' as 'ref' can be used to take references to a value

    assert_eq!(*r1, *r2);
    
    // Check the equality of the two address strings
    assert_eq!(get_addr(r1),get_addr(r2));

    println!("Success!");
}

// Get memory address string
fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}
