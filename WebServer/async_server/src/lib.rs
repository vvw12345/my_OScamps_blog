use std::{
    collections::HashMap, future::Future, pin::Pin, sync::{mpsc::{self, Sender}, Arc, Condvar, Mutex}, task::{Context, Poll, RawWaker, RawWakerVTable, Waker}, thread, time::Duration
};

#[derive(Default)]
pub struct Parker(Mutex<bool>, Condvar);

// 实现线程的暂停和唤醒
impl Parker {
    // 将当前线程暂停 直到被唤醒
    fn park(&self) {
        // 获得死锁并取出值   
        let mut resumable = self.0.lock().unwrap();
        while !*resumable {
            resumable = self.1.wait(resumable).unwrap();
        }
        *resumable = false;
    }

    // 唤醒一个之前被暂停的线程
    fn unpark(&self) {
        *self.0.lock().unwrap() = true;
        // 用信号量机制唤醒一个被阻塞的线程
        self.1.notify_one();
    }
}

// 阻塞等待一个Future直到其完成
pub fn block_on<F: Future>(mut future: F) -> F::Output {
    let parker = Arc::new(Parker::default());
    let mywaker = Arc::new(MyWaker { parker: parker.clone() });
    let waker = mywaker_into_waker(Arc::into_raw(mywaker));
    let mut cx = Context::from_waker(&waker);
    
    let mut future = unsafe { Pin::new_unchecked(&mut future) };
    loop {
        match Future::poll(future.as_mut(), &mut cx) {
            Poll::Ready(val) => break val,
            Poll::Pending => parker.park(),
        };
    }
}

// 自定义的waker结构体，用于和Parker一起工作
pub struct MyWaker {
    parker: Arc<Parker>,
}

// 唤醒 MyWaker，解除相关线程的暂停状态。
pub fn mywaker_wake(s: &MyWaker) {
    let waker_arc = unsafe { Arc::from_raw(s) };
    waker_arc.parker.unpark();
}

// 克隆 MyWaker 的函数。
pub fn mywaker_clone(s: &MyWaker) -> RawWaker {
    let arc = unsafe { Arc::from_raw(s) };
    std::mem::forget(arc.clone());
    RawWaker::new(Arc::into_raw(arc) as *const (), &VTABLE)
}

// 定义 RawWaker 的行为的 RawWakerVTable。
const VTABLE: RawWakerVTable = RawWakerVTable::new(
    |s| mywaker_clone(unsafe { &*(s as *const MyWaker) }),
    |s| mywaker_wake(unsafe { &*(s as *const MyWaker) }),
    |s| mywaker_wake(unsafe { *(s as *const &MyWaker) }),
    |s| drop(unsafe { Arc::from_raw(s as *const MyWaker) }),
);

// 将 MyWaker 转换为 Waker 的函数。
pub fn mywaker_into_waker(s: *const MyWaker) -> Waker {
    let raw_waker = RawWaker::new(s as *const (), &VTABLE);
    unsafe { Waker::from_raw(raw_waker) }
}

#[derive(Clone)]
pub struct Task {
    id: usize,
    reactor: Arc<Mutex<Reactor>>,
    data: u64,
}

impl Task {
    pub fn new(reactor: Arc<Mutex<Reactor>>, data: u64, id: usize) -> Self {
        Task { id, reactor, data }
    }
}

// 为 Task 实现 Future trait。
impl Future for Task {
    type Output = usize;
    // poll 方法用于检查任务是否准备就绪。
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut r = self.reactor.lock().unwrap();
        if r.is_ready(self.id) {
            *r.tasks.get_mut(&self.id).unwrap() = TaskState::Finished;
            Poll::Ready(self.id)
        } else if r.tasks.contains_key(&self.id) {
            r.tasks.insert(self.id, TaskState::NotReady(cx.waker().clone()));
            Poll::Pending
        } else {
            r.register(self.data, cx.waker().clone(), self.id);
            Poll::Pending
        }
    }
}

// 枚举，表示任务的状态。
pub enum TaskState {
    Ready,
    NotReady(Waker),
    Finished,
}

// Reactor 结构体，用于管理和分发事件。
pub struct Reactor {
    dispatcher: Sender<Event>,
    handle: Option<thread::JoinHandle<()>>,
    tasks: HashMap<usize, TaskState>,
}

// 枚举，表示不同类型的事件。
pub enum Event {
    Close,
    Timeout(u64, usize),
}

impl Reactor {
    // 创建一个新的 Reactor 实例。
    pub fn new() -> Arc<Mutex<Self>> {
        let (tx, rx) = mpsc::channel::<Event>();
        let reactor = Arc::new(Mutex::new(Reactor {
            dispatcher: tx,
            handle: None,
            tasks: HashMap::new(),
        }));
        
        let reactor_clone = Arc::downgrade(&reactor);
        let handle = thread::spawn(move || {
            let mut handles = vec![];
            for event in rx {
                let reactor = reactor_clone.clone();
                match event {
                    Event::Close => break,
                    Event::Timeout(duration, id) => {
                        let event_handle = thread::spawn(move || {
                            thread::sleep(Duration::from_secs(duration));
                            let reactor = reactor.upgrade().unwrap();
                            reactor.lock().map(|mut r| r.wake(id)).unwrap();
                        });
                        handles.push(event_handle);
                    }
                }
            }
            handles.into_iter().for_each(|handle| handle.join().unwrap());
        });
        reactor.lock().map(|mut r| r.handle = Some(handle)).unwrap();
        reactor
    }

    // 唤醒一个指定 id 的任务。
    fn wake(&mut self, id: usize) {
        let state = self.tasks.get_mut(&id).unwrap();
        match std::mem::replace(state, TaskState::Ready) {
            TaskState::NotReady(waker) => waker.wake(),
            TaskState::Finished => panic!("Called 'wake' twice on task: {}", id),
            _ => unreachable!(),
        }
    }

    // 注册一个新的任务，包括其持续时间和 waker。
    fn register(&mut self, duration: u64, waker: Waker, id: usize) {
        if self.tasks.insert(id, TaskState::NotReady(waker)).is_some() {
            panic!("Tried to insert a task with id: '{}', twice!", id);
        }
        self.dispatcher.send(Event::Timeout(duration, id)).unwrap();
    }

    // 关闭Reactor
    pub fn close(&mut self) {
        self.dispatcher.send(Event::Close).unwrap();
    }

    // 检查一个任务是否已经准备就绪。
    fn is_ready(&self, id: usize) -> bool {
        self.tasks.get(&id).map(|state| match state {
            TaskState::Ready => true,
            _ => false,
        }).unwrap_or(false)
    }
}

// 为 Reactor 实现 Drop trait，确保正确清理。
impl Drop for Reactor {
    fn drop(&mut self) {
        self.handle.take().map(|h| h.join().unwrap()).unwrap();
    }
}
