use std::net::{TcpListener, TcpStream};
use std::io::{self, Read, Write};
use std::sync::{Arc, Mutex};
use std::thread;

fn main() -> io::Result<()> {
    // 创建 TCP 监听器
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    println!("服务器运行中...");

    // 循环接受连接
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let _client_addr = stream.peer_addr().unwrap();
                //println!("客户端 {} 连接成功", client_addr);

                // 为每个客户端创建一个线程
                let client_stream = Arc::new(Mutex::new(stream));
                thread::spawn(move || {
                    handle_client(client_stream);
                });
            }
            Err(e) => {
                // 错误处理
                eprintln!("连接失败: {}", e);
            }
        }
    }

    Ok(())
}

fn handle_client(client_stream: Arc<Mutex<TcpStream>>) {
    let mut buffer = [0; 1024];
    loop {
        let mut stream = client_stream.lock().unwrap();
        match stream.read(&mut buffer) {
            Ok(size) if size == 0 => {
                // 对端关闭连接
                //println!("客户端断开连接");
                break;
            },
            Ok(size) => {
                let message= String::from_utf8_lossy(&buffer[..size]);
                //println!("收到消息: {}", message);

                // 发送响应
                let response = format!("服务器响应: {}", message);
                stream.write_all(response.as_bytes()).unwrap();
            },
            Err(e) => {
                // 处理错误
                eprintln!("读取失败: {}", e);
                break;
            }
        }
    }
}