// From/Into problem solution
// 1st method:
fn main() {
     // impl From<bool> for i32
    let i1:i32 = false.into();
    let i2:i32 = i32::from(false);  
    assert_eq!(i1, i2);
    assert_eq!(i1, 0);

    let i3: u32 = 'a'.into();  // used 'u32' data type instead of 'i32'

    let s: String = 'a'.into();
    
    println!("Success!");
}

// 2nd method:
fn main() {
     // impl From<bool> for i32
    let i1:i32 = false.into();
    let i2:i32 = i32::from(false);  
    assert_eq!(i1, i2);
    assert_eq!(i1, 0);

    let i3: u32 = 'a' as u32 ;  // used 'u32' data type instead of 'i32' and replaced 'a'.into() to 'a' as u32

    let s: String = String::from('a');
    
    println!("Success!");
}