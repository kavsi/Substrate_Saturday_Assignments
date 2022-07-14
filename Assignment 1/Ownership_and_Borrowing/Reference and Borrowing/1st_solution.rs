//Reference problem solution
fn main() {
   let x = 5;  
   let p = &x;   // &x syntax creates a reference that refers to the memory address of x

   println!("the memory address of x is {:p}", p); // One possible output: 0x16fa3ac84
}
