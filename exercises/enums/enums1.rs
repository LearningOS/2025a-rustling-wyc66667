// enums1.rs
//
// No hints this time! ;)

// I AM NOT DONE

#[derive(Debug)]
enum Message {
    // 定义枚举的四个变体，对应main函数中使用的消息类型
    Quit,
    Echo,
    Move,
    ChangeColor,
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
