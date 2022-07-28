// binary, octal, hex problem solution
fn main() {
     assert_eq!(format!("{:#b}", 27), "0b11011");  // the blank is filled with '{:#b}'
     assert_eq!(format!("{:#o}", 27), "0o33");     // the blank is filled with '{:#o}'
     assert_eq!(format!("{:#x}", 27), "0x1b");     // the blank is filled with '{:#x}'
     assert_eq!(format!("{:#X}", 27), "0x1B");     // the blank is filled with '{:#X}'

     println!("{:x}!", 27); // hex with no prefix => 1b
 
     println!("{:#010b}", 27); // pad binary with 0, width = 10,  => 0b00011011

    println!("Success!")
}