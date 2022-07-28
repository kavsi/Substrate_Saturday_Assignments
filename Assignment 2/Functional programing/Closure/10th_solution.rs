// Closure as return types problem solution
// 1st method:
/* Fill in the blank and fix the errror */
fn create_fn() -> impl Fn(i32) -> i32 {  // the blank is filled with 'impl Fn(i32) -> i32' and we can also use `impl FnOnce(i32) -> i32`
    let num = 5;

    move |x| x + num
}


fn main() {
    let fn_plain = create_fn();
    fn_plain(1);
}

// 2nd method:
/* Fill in the blank and fix the errror */
fn create_fn() -> Box<dyn Fn(i32) -> i32> {
    let num = 5;

    // how does the following closure capture the evironment variable `num`
    // &T, &mut T, T ?
    Box::new(move |x| x + num)
}


fn main() {
    let fn_plain = create_fn();
    fn_plain(1);
}