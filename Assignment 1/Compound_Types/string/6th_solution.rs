//String problem solution
fn main() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1.clone() + &s2;         // 's1' is cloned and 's2' is replaced with '&s2'
    assert_eq!(s3,"hello,world!");
    println!("{}",s1);
}
