use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex}, time::Instant,
};

use my_server::{block_on, Reactor, Task};

fn main() {
    let start = Instant::now();
    let reactor = Reactor::new();

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let reactor_clone = reactor.clone();

        let fut = async move {
            handle_connection(stream, reactor_clone).await;
            println!("Connection handled at time: {:.2}.", start.elapsed().as_secs_f32());
        };

        block_on(fut);
    }

    reactor.lock().map(|mut r| r.close()).unwrap();
}

async fn handle_connection(mut stream: TcpStream, _reactor: Arc<Mutex<Reactor>>) {
    let mut buf_reader = BufReader::new(&mut stream);
    
    // 读取客户端发送的请求行（如果有的话），但在此示例中我们不处理它
    if let Some(_) = buf_reader.lines().next() {
        // 忽略请求行
    }

    // 准备一个缓冲区来接收数据
    let mut buffer = [0; 1024];

    // 持续对话循环
    loop {
        match stream.read(&mut buffer) {
            Ok(size) if size == 0 => {
                // 客户端关闭连接
                println!("Client disconnected.");
                break;
            },
            Ok(size) => {
                // 客户端发送了数据，打印并回显
                let request = String::from_utf8_lossy(&buffer[..size]);
                println!("Received from client: {}", request);

                // 将接收到的数据发送回客户端
                stream.write_all(&buffer[..size]).unwrap();
                stream.flush().unwrap(); // 确保数据被发送出去
            },
            Err(e) => {
                // 读取数据时发生错误
                eprintln!("Error reading from stream: {}", e);
                break;
            }
        }
    }
}
