// Fn, FnMut, FnOnce problem solution
// 1st method:
fn fn_once<F>(func: F)
where
    F: Fn(usize) -> bool,  // replaced 'FnOnce' with 'fn'
{
    println!("{}", func(3));
    println!("{}", func(4));
}

fn main() {
    let x = vec![1, 2, 3];
    fn_once(|z|{z == x.len()})
}

//2nd method:
fn fn_once<F>(func: F)
where
    F: FnOnce(usize) -> bool + Copy,  // added 'copy' to the line
{
    println!("{}", func(3));
    println!("{}", func(4));
}

fn main() {
    let x = vec![1, 2, 3];
    fn_once(|z|{z == x.len()})
}