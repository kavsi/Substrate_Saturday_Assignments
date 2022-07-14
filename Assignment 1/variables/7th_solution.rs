//Shadowing problem solution
//1st method:
fn main() {
    let _x = 1;               //Ignoring an Unused Variable by Starting Its Name with _
}

//2nd method:
#[allow(unused_variables)]    //used #[allow(unused_variables)] to allow unused variables.
fn main() {
    let x = 1; 
}
