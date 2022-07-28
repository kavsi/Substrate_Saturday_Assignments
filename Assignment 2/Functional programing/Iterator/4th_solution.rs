// into_iter, iter and iter_mut problem solution
fn main() {
    let arr = vec![0; 10];
    for i in arr.iter() {  // here '.iter()' is used for iteration
        println!("{}", i)
    }

    println!("{:?}",arr);
}