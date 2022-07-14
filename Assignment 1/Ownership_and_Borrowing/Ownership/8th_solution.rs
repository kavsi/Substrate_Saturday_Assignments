//Partial move Exercises solution
fn main() {
   let t = (String::from("hello"), String::from("world"));

   let _s = t.0;

   // Modify this line only, don't use `_s`
   println!("{:?}", t.1);          // As t.0 is already used, we used t.1 here
}
