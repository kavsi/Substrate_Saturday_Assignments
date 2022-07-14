//Slice problem solution
fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    
    let slice: &[i32] = &arr[1..4];  // the blanks are filled with '&[i32]' and '&arr[1..4]'
    assert_eq!(slice, &[2, 3, 4]);

    println!("Success!");
}