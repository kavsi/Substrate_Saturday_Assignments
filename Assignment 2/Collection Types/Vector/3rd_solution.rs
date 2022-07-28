// Turn X Into Vec problem solution
fn main() {
    // array -> Vec
    let arr = [1, 2, 3];
    let v1 = Vec::from(arr);  // the blank is  filled with 'Vec::from'
    let v2: Vec<i32> = arr.into();  // the blank is  filled with 'into()'
 
    assert_eq!(v1, v2);
 
    
    // String -> Vec
    // impl From<String> for Vec
    let s = "hello".to_string();
    let v1: Vec<u8> = s.into();   // the blank is  filled with 'into()'

    let s = "hello".to_string();
    let v2 = s.into_bytes();
    assert_eq!(v1, v2);

    // impl<'_> From<&'_ str> for Vec
    let s = "hello";
    let v3 = Vec::from(s);  // the bank is filed with 'from'
    assert_eq!(v2, v3);

    println!("Success!")
 }