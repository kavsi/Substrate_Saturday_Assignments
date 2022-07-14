//scope problem solution
//1st method:
fn main() {
    let x = define_x();           //x takes value from define_x() function.
    println!("{}, world", x);
}

fn define_x() -> String {         //return type String.
    let x = "hello".to_string();  //to_string() will let variable to take string input.
    return x;
}

//2nd method:
fn main() {
    let x = define_x();            //x takes value from define_x() function.
    println!("{}, world", x); 
}

fn define_x() -> String {          //return type String.
    let x = String::from("hello"); //string::from("input") is another way to assign string value to x.
    return x;
}
