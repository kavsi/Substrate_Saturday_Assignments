//Array problem solution
fn main() {

    let list: [i32; 100] = [1;100];   // the blank is filled with [1;100] as the first element is 1 and length is 100

    assert!(list[0] == 1);
    assert!(list.len() == 100);

    println!("Success!");
}
