//Mutability problem solution
ffn main() {
    let x = Box::new(5);           // Boxes allow us to store data on the heap rather than the stack
    
    let  mut y = Box::new(4);      // filled this line.
    
    *y = 4;
    
    assert_eq!(*x, 5);

    println!("Success!");
}
