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

async fn handle_connection(mut stream: TcpStream, reactor: Arc<Mutex<Reactor>>) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            Task::new(reactor.clone(), 5, 1).await; // 模拟异步等待
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}
