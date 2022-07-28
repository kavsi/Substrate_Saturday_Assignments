// Input functions problem solution
fn call_me<F: Fn()>(f: F) {  // Define a function which takes a generic `F` argument bounded by `Fn`, and calls it
    f();
}

fn function() {   // Define a wrapper function satisfying the `Fn` bound
    println!("I'm a function!");
}

fn main() {
    let closure = || println!("I'm a closure!");   // Define a closure satisfying the `Fn` bound

    call_me(closure);
    call_me(function);
}