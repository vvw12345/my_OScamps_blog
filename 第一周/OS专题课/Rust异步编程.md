# Rust相关基础补全

## 多线程编程

### 并发模型

如果大家学过其它语言的多线程，可能就知道不同语言对于线程的实现可能大相径庭：

- 由于操作系统提供了创建线程的 API，因此部分语言会直接调用该 API 来创建线程，因此最终程序内的线程数和该程序占用的操作系统线程数相等，一般称之为**1:1 线程模型**，例如 Rust。
- 还有些语言在内部实现了自己的线程模型（绿色线程、协程），程序内部的 M 个线程最后会以某种映射方式使用 N 个操作系统线程去运行，因此称之为**M:N 线程模型**，其中 M 和 N 并没有特定的彼此限制关系。一个典型的代表就是 Go 语言。（显著增加运行时大小）
- 还有些语言使用了 Actor 模型，基于消息传递进行并发，例如 Erlang 语言。

### 使用多线程

`thread::spawn`

`thread::sleep`	

`handle.join()` 阻塞当前线程

> RUST线程如何结束？
>
> **线程的代码执行完，线程就会自动结束!!!**
>
> 执行不完呢？
>
> 分两种情况 一种是轮询I/O 大多数时候其实是阻塞的
>
> 另外一种是死循环 那CPU会跑满……

`call_once()`在特定线程只会被执行一次的函数，如果后续再调用该进程，这部分会被直接跳过

#### 线程屏障

```rust
use std::sync::{Arc, Barrier};
use std::thread;

fn main() {
    let mut handles = Vec::with_capacity(6);
    let barrier = Arc::new(Barrier::new(6));

    for _ in 0..6 {
        let b = barrier.clone();
        handles.push(thread::spawn(move|| {
            println!("before wait");
            b.wait(); //所有线程执行到该位置停下，等待其他所有线程
            println!("after wait");
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
```



#### 多线程的move

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];
	
    // move关键字先取走v的所有权 再创建线程
    // 新线程不确定主线程可以活多久 
    // 有可能主线程结束 v被释放了 新线程还没创建出来
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();

    // 下面代码会报错borrow of moved value: `v`
    // println!("{:?}",v);
}
```

#### 线程局部变量

```rust
use std::cell::RefCell;
use std::thread;

// 创建线程局部变量
thread_local!(static FOO: RefCell<u32> = RefCell::new(1));

//线程局部变量的with()方法可在当前线程获得该局部变量的值
FOO.with(|f| {
    assert_eq!(*f.borrow(), 1);
    *f.borrow_mut() = 2;
});

// 每个线程开始时都会拿到线程局部变量的FOO的初始值
let t = thread::spawn(move|| {
    FOO.with(|f| {
        assert_eq!(*f.borrow(), 1);
        *f.borrow_mut() = 3;
    });
});

// 等待线程完成
t.join().unwrap();

// 尽管子线程中修改为了3，我们在这里依然拥有main线程中的局部值：2
FOO.with(|f| {
    assert_eq!(*f.borrow(), 2);
});

```





#### 多线程性能

性能并非随线程数量线性增长 反而有可能下降

- 线程过多时，CPU 缓存的命中率会显著下降，同时多个线程竞争一个 CPU Cache-line 的情况也会经常发生
- 大量读写可能会让内存带宽也成为瓶颈
- 读和写不一样，无锁数据结构的读往往可以很好地线性增长，但是写不行，因为写竞争太大![img](https://pic3.zhimg.com/80/v2-af225672de09c0e377023f5f39dd87eb_1440w.png)



### 线程同步

#### 消息传递

**一个消息通道只能传输一种类型的数据**，如果你想要传输多种类型的数据，可以为每个类型创建一个通道，也可以用枚举把消息都封装起来。

`mpsc`库   *multiple producer, single consumer*

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    // 创建一个消息通道, 返回一个元组：(发送者，接收者)
    let (tx, rx) = mpsc::channel();

    // 创建线程，并发送消息
    thread::spawn(move || {
        // 发送一个数字1, send方法返回Result<T,E>，通过unwrap进行快速错误处理
        // 也有可能返回Error(可能是因为找不到接收者)
        tx.send(1).unwrap();

        // 下面代码将报错，因为编译器自动推导出通道传递的值是i32类型，那么Option<i32>类型将产生不匹配错误
        // tx.send(Some(1)).unwrap()
    });

    // 接收消息的操作rx.recv()会阻塞当前线程，直到读取到值，或者通道被关闭
    println!("receive {}", rx.recv().unwrap());
    // try_recv()方法 尝试接收一次消息 该方法不会阻塞线程
    // 在本例中是会失败的 因为创建新线程的速度远比执行到这里开始尝试接收消息要慢的多
    println!("receive {:?}", rx.try_recv());
}
```

传输具有所有权的数据呢？

看有没有`Copy`特征，如果有的话就直接拷贝一份过去，没有的话就转移其所有权



如果是多发送者的情况呢？

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    // 子线程执行发送会拿走发送者的所有权
    // 所以需要对发送者建立一份拷贝
    thread::spawn(move || {
        tx.send(String::from("hi from raw tx")).unwrap();
    });

    thread::spawn(move || {
        tx1.send(String::from("hi from cloned tx")).unwrap();
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
```



同步通道和异步通道的差别

`mpsc::channel()` 创建通道（异步） 无论接收者是否正在接收消息，消息发送者在发送消息时都不会阻塞

`mpsc::sync_channel(N) `同步通道  **发送消息是阻塞的，只有在消息被接收后才解除阻塞** 

N为消息缓存大小  不超过消息缓存时可以无阻塞发送 大于缓存之后需要有人接收才可以解除阻塞



通道结束条件

**所有发送者被`drop`或者所有接收者被`drop`后，通道会自动关闭**

```rust
use std::sync::mpsc;
fn main() {

    use std::thread;

    let (send, recv) = mpsc::channel();
    let num_threads = 3;
    for i in 0..num_threads {
        //每个线程建立一个发送者拷贝 加上原来的总共有四个send
        let thread_send = send.clone();
        thread::spawn(move || {
            thread_send.send(i).unwrap();
            println!("thread {:?} finished", i);
        });
    }

    // 在这里drop send...
    //这里不drop send的话 那还剩下一个发送者
    //接下来的for循环永远不会结束

    for x in recv { 
        println!("Got: {}", x);
    }
    println!("finished iterating");
}
```



#### 锁

```rust
use std::sync::Mutex;

fn main() {
    // 使用`Mutex`结构体的关联函数创建新的互斥锁实例
    let m = Mutex::new(5);

    {
        // 获取锁 lock()方法会阻塞当前线程 直到获取到锁
        // lock返回的是Result Result内部是个智能指针MutexGuard<T>
        // 实现了Deref特征和drop特征
        let mut num = m.lock().unwrap();
        *num = 6;
        // 锁自动被drop
    }

    println!("m = {:?}", m);
}
```

`try_lock()` 尝试获取一次锁 不会发生阻塞

`RwLock`读写锁 允许多个读  其余情况阻塞（读写不能同时存在

```rust
use std::sync::RwLock;

fn main() {
    let lock = RwLock::new(5);

    // 同一时间允许多个读
    {
        let r1 = lock.read().unwrap();
        let r2 = lock.read().unwrap();
        assert_eq!(*r1, 5);
        assert_eq!(*r2, 5);
    } // 读锁在此处被drop

    // 同一时间只允许一个写
    {
        let mut w = lock.write().unwrap();
        *w += 1;
        assert_eq!(*w, 6);

        // 以下代码会阻塞发生死锁，因为读和写不允许同时存在
        // 写锁w直到该语句块结束才被释放，因此下面的读锁依然处于`w`的作用域中
        // let r1 = lock.read();
        // println!("{:?}",r1);
    }// 写锁在此处被drop
}
```

首先简单性上`Mutex`完胜，因为使用`RwLock`你得操心几个问题：

- 读和写不能同时发生，如果使用`try_xxx`解决，就必须做大量的错误处理和失败重试机制
- 当读多写少时，写操作可能会因为一直无法获得锁导致连续多次失败([writer starvation](https://stackoverflow.com/questions/2190090/how-to-prevent-writer-starvation-in-a-read-write-lock-in-pthreads))
- RwLock 其实是操作系统提供的，实现原理要比`Mutex`复杂的多，因此单就锁的性能而言，比不上原生实现的`Mutex`

再来简单总结下两者的使用场景：

- 追求高并发读取时，使用`RwLock`，因为`Mutex`一次只允许一个线程去读取
- 如果要保证写操作的成功性，使用`Mutex`
- 不知道哪个合适，统一使用`Mutex`

需要注意的是，`RwLock`虽然看上去貌似提供了高并发读取的能力，但这个不能说明它的性能比`Mutex`高，事实上`Mutex`性能要好不少，后者**唯一的问题也仅仅在于不能并发读取**。

一个常见的、错误的使用`RwLock`的场景就是使用`HashMap`进行简单读写，因为`HashMap`的读和写都非常快，`RwLock`的复杂实现和相对低的性能反而会导致整体性能的降低，因此一般来说更适合使用`Mutex`。

总之，如果你要使用`RwLock`要确保满足以下两个条件：**并发读，且需要对读到的资源进行"长时间"的操作**，`HashMap`也许满足了并发读的需求，但是往往并不能满足后者："长时间"的操作。

#### Atomic原子类型

性能优于锁和消息传递

多核环境下执行会停止其他CPU对内存的操作

```rust
use std::ops::Sub;
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread::{self, JoinHandle};
use std::time::Instant;

const N_TIMES: u64 = 10000000;
const N_THREADS: usize = 10;

// Atomic类型天生具有内部可变性
static R: AtomicU64 = AtomicU64::new(0);

fn add_n_times(n: u64) -> JoinHandle<()> {
    thread::spawn(move || {
        for _ in 0..n {
            R.fetch_add(1, Ordering::Relaxed);// Ordering::Relaxed内存顺序限定
        }
    })
}

fn main() {
    let s = Instant::now();
    let mut threads = Vec::with_capacity(N_THREADS);

    for _ in 0..N_THREADS {
        threads.push(add_n_times(N_TIMES));
    }

    for thread in threads {
        thread.join().unwrap();
    }

    assert_eq!(N_TIMES * N_THREADS as u64, R.load(Ordering::Relaxed));

    println!("{:?}",Instant::now().sub(s));
}
```

>  内存顺序问题起源：编译器优化 CPU缓存……
>
> 内存顺序规则：relexed   release   acquire  AcqRel  SeqCst

能否替代锁？

- 对于复杂的场景下，锁的使用简单粗暴，不容易有坑
- `std::sync::atomic`包中仅提供了数值类型的原子操作：`AtomicBool`, `AtomicIsize`, `AtomicUsize`, `AtomicI8`, `AtomicU16`等，而锁可以应用于各种类型
- 在有些情况下，必须使用锁来配合，例如上一章节中使用`Mutex`配合`Condvar`



#### Send和Sync特征

标记特征

`send` 线程间所有权传递

`sync` 线程间通过引用共享

绝大多数类型都实现了`send`和`sync` 只有极少数没有……裸指针，RC等

自定义复合类型中任意成员未实现 则复合类型也实现不了

```rust
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Debug)]
struct MyBox(*const u8);
unsafe impl Send for MyBox {}//MyBox内部是个裸指针 所以默认实现不了 手动为其实现
unsafe impl Sync for MyBox {}//手动实现Sync使其能够解引用

fn main() {
    let b = &MyBox(5 as *const u8);
    let v = Arc::new(Mutex::new(b));
    let t = thread::spawn(move || {
        let _v1 =  v.lock().unwrap();
    });

    t.join().unwrap();
}
```







## UnSafe



## 异步编程

