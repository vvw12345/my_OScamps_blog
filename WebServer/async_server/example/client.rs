use std::fs::File;
use std::io::{BufWriter, Read, Write};
use std::net::TcpStream;
use std::path::Path;
use std::thread;
use std::time::Instant;

fn main() {
    // 设置要测试的客户端数量范围
    let start_client_count = 0;
    let end_client_count = 10000;
    let step = 10; // 每次增加的数量

    // 设置服务器地址和输出文件路径
    let addr = "127.0.0.1:8080";
    let output_file_path = "test_results.csv"; // CSV 文件用于存储结果

    // 用于存储测试结果的向量
    let mut results = Vec::new();

    for client_count in (start_client_count..=end_client_count).step_by(step) {
        // 创建一个向量来存储所有的线程句柄
        let mut handles = vec![];

        // 记录开始时间
        let start_time = Instant::now();

        for id in 0..client_count {
            let addr_clone = addr.to_string();
            let client_id = id;

            // 创建一个新的线程来处理客户端逻辑
            let handle = thread::spawn(move || {
                if let Err(e) = client_interaction(addr_clone, client_id) {
                    eprintln!("客户端 {} 出错: {}", client_id + 1, e);
                }
            });

            handles.push(handle);
        }

        // 等待所有线程完成
        for handle in handles {
            handle.join().unwrap();
        }

        // 记录结束时间
        let duration = start_time.elapsed();

        // 保存单次测试结果
        results.push((client_count, duration.as_secs_f64()));

        // 可以选择在这里打印出单次测试的结果
        println!("客户端数量: {}, 耗时: {:?}", client_count, duration);
    }

    // 所有测试完成后，将结果保存到文件
    save_test_results(output_file_path, &results);
}

fn client_interaction(addr: String, id: usize) -> std::io::Result<()> {
    let mut stream = TcpStream::connect(addr)?;

    // 每个客户端会给服务器发送大约10条消息
    for message_number in 1..=10 {
        let message = format!("Client {} says: {}", id + 1, message_number);
        stream.write_all(message.as_bytes())?;

        // 等待服务器响应
        let mut buffer = vec![0; 1024];
        stream.read(&mut buffer)?;
    }

    Ok(())
}

fn save_test_results<P: AsRef<Path>>(path: P, results: &[(usize, f64)]) {
    let mut file = BufWriter::new(File::create(path).unwrap());

    // 写入标题行
    writeln!(file, "client_count,total_duration").unwrap();

    // 写入测试结果
    for (client_count, duration) in results {
        writeln!(file, "{},{}", client_count, duration).unwrap();
    }
}