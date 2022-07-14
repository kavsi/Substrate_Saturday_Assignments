//Reference problem solution
fn main() {
    let mut s = String::from("hello, ");

    borrow_object(&s);          // used '&' because function uses '&' to indicate that the type of the parameter 's' is a reference

    println!("Success!");
}

fn borrow_object(s: &String) {}    
