use tokio::net::{TcpListener, TcpStream};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> io::Result<()> {
    // 创建 TCP 监听器
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("服务器运行中...");

    // 循环接受连接
    while let Ok((socket, addr)) = listener.accept().await {
        //println!("客户端 {} 连接成功", addr);
        // 为每个客户端创建一个异步任务
        tokio::spawn(async move {
            handle_client(socket).await;
        });
    }
    Ok(())
}

async fn handle_client(mut socket: TcpStream) {
    let mut buffer = [0; 1024];
    loop {
        // 使用socket.read()异步的读取数据
        match socket.read(&mut buffer).await {
            Ok(size) if size == 0 => {
                // 对端关闭连接
                break;
            },
            Ok(size) => {
                let message = String::from_utf8_lossy(&buffer[..size]);
                //println!("收到消息: {}", message);

                // 发送响应(这里的响应是自动响应)
                let response = format!("服务器响应: {}", message);
                socket.write_all(response.as_bytes()).await.unwrap();
                
                // 之前留下的内容 支持手动输入消息发送
                // let mut response = String::new();
                // println!("请输入消息: ");
                // std::io::stdin().read_line(&mut response).unwrap();
            },
            Err(e) => {
                // 处理错误
                eprintln!("读取失败: {}", e);
                break;
            }
        }
    }
}

// 实现业务逻辑函数，例如 register, login_password 等...

