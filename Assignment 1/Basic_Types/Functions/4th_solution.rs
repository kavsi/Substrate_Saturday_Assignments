//Diverging functions problem solution
//1st method:
fn main() {
    println!("Success!");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            // TODO
        }
        _ => {
            // TODO
        }
    };
    
    // Rather than returning a None, we use a diverging function instead
    never_return_fn()
}

fn never_return_fn() -> ! {
    panic!()    // creating panic!() 
}


//2nd method:
fn main() {
    println!("Success!");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            // TODO
        }
        _ => {
            // TODO
        }
    };
    
    // Rather than returning a None, we use a diverging function instead
    never_return_fn()
}

fn never_return_fn() -> ! {
    unimplemented!()    // creating unimplemented!() 
}

//3rd method:
fn main() {
    println!("Success!");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            // TODO
        }
        _ => {
            // TODO
        }
    };
    
    // Rather than returning a None, we use a diverging function instead
    never_return_fn()
}

fn never_return_fn() -> ! {
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1)) // sleeping for 1 second to avoid exhausting the cpu resoucre
    }
}