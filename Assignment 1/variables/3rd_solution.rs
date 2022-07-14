//scope problem
fn main() {
    let x: i32 = 10;
    {
        let y: i32 = 5;                      //the value of variable 'y' is defined only in the inner scope
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {}", x);     //removed variable 'y' and its value from printing.
}

