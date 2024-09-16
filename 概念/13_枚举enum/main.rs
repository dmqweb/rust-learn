#![allow(dead_code)]
#[derive(Debug)] //为Message枚举实现debug trait
enum Message {
    Resize { width: u64, height: u64 },
    Move(Point),
    Echo(String),
    ChangeColor(u8, u8, u8),
    Quit,
}
#[derive(Debug)] //为Message结构体实现debug trait
struct Point {
    x: u64,
    y: u64,
}
#[derive(Debug)] //为State结构体实现debug trait
struct State {
    width: u64,
    height: u64,
    position: Point,
    message: String,
    color: (u8, u8, u8),
    quit: bool,
}
// 使用impl实现
impl State {
    fn resize(&mut self, width: u64, height: u64) {
        self.width = width;
        self.height = height;
    }
    fn move_position(&mut self, point: Point) {
        self.position = point;
    }
    fn echo(&mut self, s: String) {
        self.message = s;
    }
    fn change_color(&mut self, red: u8, green: u8, blue: u8) {
        self.color = (red, green, blue);
    }
    fn quit(&mut self) {
        self.quit = true;
    }
    fn process(&mut self, message: Message) {
        match message {
            Message::Resize { width, height } => self.resize(width, height),
            Message::Move(point) => self.move_position(point),
            Message::Echo(s) => self.echo(s),
            Message::ChangeColor(red, green, blue) => self.change_color(red, green, blue),
            Message::Quit => self.quit(),
        }
    }
}
fn main() {
    let mut state = State {
        width: 0,
        height: 0,
        position: Point { x: 0, y: 0 },
        message: String::from("hello world"),
        color: (0, 0, 0),
        quit: false,
    };
    println!("创建的state为：{:?}", state);
    state.process(Message::Resize {
        width: 10,
        height: 30,
    });
    state.process(Message::Move(Point { x: 10, y: 15 }));
    state.process(Message::Echo(String::from("Hello world!")));
    state.process(Message::ChangeColor(255, 0, 255));
    state.process(Message::Quit);
    println!("{}--{}", state.width, 10);
    println!("{}--{}", state.height, 30);
    println!("{}--{}", state.position.x, 10);
    println!("{}--{}", state.position.y, 15);
    println!("{}--{}", state.message, "Hello world!");
    // println!("{}{}", state.color, (255, 0, 255));
    assert!(state.quit);
}
