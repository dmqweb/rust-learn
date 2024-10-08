use std::fs; //读取html文件
use std::io::prelude::*;
use std::net::TcpListener; //建立TCP连接
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // incoming方法返回迭代器
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let contents = fs::read_to_string("./index.html").expect("找不到index.html");

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
