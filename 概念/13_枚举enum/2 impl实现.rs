#![allow(dead_code)] //忽略未使用的代码警告
#[derive(Debug)] //为Point实现debug的trait
struct Point {
    x: u64,
    y: u64,
}
#[derive(Debug)] //实现debug的trait
                 //创建enum枚举类型
enum Message {
    Resize { width: u64, height: u64 },
    Move(Point),
    Echo(String),
    ChangeColor(u64, u64, u64),
    Quit,
}
//使用impl实现枚举类型
impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}
fn main() {
    let messages = [
        Message::Resize {
            width: 10,
            height: 30,
        },
        Message::Move(Point { x: 10, y: 15 }),
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 200, 255),
        Message::Quit,
    ];
    for message in &messages {
        message.call();
    }
}
