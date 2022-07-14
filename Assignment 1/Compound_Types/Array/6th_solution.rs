//Array problem solution
fn main() {
    let names = [String::from("Sunfei"), "Sunface".to_string()];
    
    // `Get` returns an Option<T>, it's safe to use
    let name0 = names.get(0).unwrap();

    let _name1 = &names[1];  // number 2 is replaced by 1 as indexing start from 0 and the second element is indexed as 1

    println!("Success!");
}