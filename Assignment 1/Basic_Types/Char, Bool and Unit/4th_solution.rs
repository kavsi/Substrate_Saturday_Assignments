//Bool problem solution
fn main() {
    let f = true;
    let t = true || false;  // replaced &&(and) operator with ||(or) operator
    assert_eq!(t, f);

    println!("Success!");
}