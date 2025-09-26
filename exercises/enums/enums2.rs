// enums2.rs
//
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

#[derive(Debug)]
enum Message {
    // 定义带有不同数据类型的枚举变体
    Move { x: i32, y: i32 },    // 结构体样式的变体
    Echo(String),               // 包含一个String的元组样式变体
    ChangeColor(i32, i32, i32), // 包含三个i32的元组样式变体
    Quit,                       // 单元变体
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
