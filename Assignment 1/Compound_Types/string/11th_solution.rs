//String index problem solution
fn main() {
    let s1 = String::from("hi,中国");
    let h = &s1[0..1];     // value changed from 'o' to '0..1' 
    assert_eq!(h, "h");

    let h1 = &s1[3..6];    // value changed from '3..5' to '3..6'
    assert_eq!(h1, "中");
    
    println!("Success!");
}
