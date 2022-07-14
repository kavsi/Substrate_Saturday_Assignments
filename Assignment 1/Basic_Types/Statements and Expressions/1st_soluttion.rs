//Statements and Expressions Exercises solution
//1st method:
fn main() {
   let v = {
       let mut x = 1;
       x+2        // the variable 'v' is assigned to return value of x+2
   };

   assert_eq!(v, 3);

   println!("Success!");
}

//2nd method:
fn main() {
   let v = {
       let mut x = 1;
       x += 2     
   };

   assert_eq!(v, ());    // As there is o return value to 'v', 3 is changed to () 

   println!("Success!");
}