use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
use::web_server::ThreadPool;


fn main() {
    // 监听地址: 127.0.0.1:8080
    // bind函数绑定8080端口 unwarp()取出bind函数返回的Result<T,E>类型的值（有可能出错）
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    // 线程池最大数量
    let pool = ThreadPool::new(4);

    // incoming()方法返回一个迭代器
    // 每次迭代从TcpListener实例中获得一个stream连接
    for stream in listener.incoming() {
        // unwrap()取出值 stream是连接的尝试过程（也不一定成功）
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    //BufReader 实现缓冲区读取
    let buf_reader = BufReader::new(&mut stream);
    // 获取HTTP请求报文
    // let http_request: Vec<_> = buf_reader
    //     .lines() //获取一个迭代器，可以对传输的内容流进行按行迭代读取
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect(); //collect()方法消费掉迭代器
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    // 打印请求格式   
    //println!("Request: {:#?}", http_request);
    
    // 编辑response消息    
    //let response = "HTTP/1.1 200 OK\r\n\r\n";

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    // write_all() 方法接收&[u8]类型作为参数 因此使用as_bytes()方法转换为字节类型
    stream.write_all(response.as_bytes()).unwrap();

    
}
