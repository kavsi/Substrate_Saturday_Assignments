Patterns problem solution
enum Message {
    Hello { id i32 },
}

fn main() {
    let msg = MessageHello { id 5 };

    match msg {
        MessageHello {
            id  id@3..=7,       here 'id@' is added
        } = println!(Found an id in range [3, 7] {}, id),
        MessageHello { id newid@(10  11  12) } = {
            println!(Found an id in another range [10, 12] {}, newid)
        }
        MessageHello { id } = println!(Found some other id {}, id),
    }
}
