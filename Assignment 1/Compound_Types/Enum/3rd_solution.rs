//Enum problem solution
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::Move{x: 1, y: 1};         // the y is initialized with 1

    if let Message::Move{x: a, y: b} = msg {     // the blank is filled with initiating x: a and y: b
        assert_eq!(a, b);
    } else {
        panic!("NEVER LET THIS RUN！");
    }
    
    println!("Success!");
} 