//Unit type problem solution
fn main() {
    let v1: () = ();   // changed unassigned variable 'v' to unit type variable 'v1'

    let v = (2, 3);
    assert_eq!(v1, implicitly_ret_unit());

    println!("Success!");
}

fn implicitly_ret_unit() {
    println!("I will return a ()");
}

// Don't use this one
fn explicitly_ret_unit() -> () {
    println!("I will return a ()");
}