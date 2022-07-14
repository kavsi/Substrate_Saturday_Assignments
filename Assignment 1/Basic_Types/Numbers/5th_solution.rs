//Integer problem solution
fn main() {
   let v1 = 247_u8 + 8;                        // u8 MAX is 255. so 251 is changed to 247
   let v2 = i8::checked_add(119, 8).unwrap();  // i8 MAX is 127. so 251 is changed to 119
   println!("{},{}",v1,v2);
}
