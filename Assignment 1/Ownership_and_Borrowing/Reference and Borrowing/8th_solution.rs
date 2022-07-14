//Mutability problem solution
fn main() {

    let  mut s = String::from("hello, ");  // added 'mut' to 's' as it needs to be mutable

    borrow_object(&mut s);

    println!("Success!");
}

fn borrow_object(s: &mut String) {}
