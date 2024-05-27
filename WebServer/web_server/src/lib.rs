use std::{sync::{mpsc, Arc, Mutex}, thread};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}


type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    // 新建线程池
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0); //最大线程数量必须大于0 否则崩溃

        // 消息通道 ThreadPool持有发送端 Worker持有接收端
        // 通过ThreadPool::execute()方法发送任务
        let (sender, receiver) = mpsc::channel();
        // Rust中的mpsc实现的是多生产者单消费者 receiver的所有权会在下面的第一次循环被传递 而receiver也没有实现Copy特性
        // 我们的目的是要让同一个receiver在同一时段只能被一个Worker执行 同时receiver需要可以被传递
        // 因此在此处使用互斥锁
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    // 执行函数 类似于Rcore里面的Spawn系统调用
    pub fn execute<F>(&self, f: F)
    where
        // FnOnce()是因为传入的闭包只需要执行一次
        // Send Trait是因为闭包需要从一个线程传递到另外一个线程
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

/// Worker和ThreadPool分离
/// 在Thread::spawn创建线程的时候往往会立即执行 但是我们不希望这样子 
/// 因此我们在ThreadPool里面创建我们的进程 在Worker里面执行
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker   {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {id} got a job; executing.");

            job();
        });

        Worker { id, thread }
    }
}