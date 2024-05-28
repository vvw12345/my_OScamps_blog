use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex}, time::Instant,
};

use async_server::{block_on, Reactor, Task};


fn main() {
    let start = Instant::now();
    let reactor = Reactor::new();

    // 绑定8080端口 监听连接
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    // 用迭代器取出连接尝试（不一定成功）
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        // Arc复制一份Reactor 
        let reactor_clone = reactor.clone();
        
        // 异步执行
        let fut = async move {
            handle_connection(stream, reactor_clone).await;
            println!("Connection handled at time: {:.2}.", start.elapsed().as_secs_f32());
        };
        block_on(fut);
    }

    reactor.lock().map(|mut r| r.close()).unwrap();
}

// 消息处理函数
async fn handle_connection(mut stream: TcpStream, reactor: Arc<Mutex<Reactor>>) {
    // 创建缓冲区 用于从流中读取数据
    let buf_reader = BufReader::new(&mut stream);
    // 从流中读出请求行
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    
    // 根据请求行确定请求状态和请求的路径
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            // 模拟异步等待 不能用Thread::sleep直接睡眠
            // Thread::sleep会阻塞掉当前的线程 导致其他任务无法继续执行
            Task::new(reactor.clone(), 5, 1).await; 
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}
