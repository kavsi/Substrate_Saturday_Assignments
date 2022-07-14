//If let problem solution
fn main() {
   
    // Remove the whole `match` block, using `if let` instead 
    let o = Some(7);
    
    if let Some(i) = o {
        println!("This is a really long string and `{:?}`", i);
    }
}