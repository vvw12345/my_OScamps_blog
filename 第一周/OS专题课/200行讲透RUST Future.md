# 200行讲透RUST Future

## 并发任务处理的主要思路

- OS线程    适合CPU密集型，少量任务并发（空闲的任务也会消耗系统资源    

  优点：不会破坏你的代码逻辑和编程模型

  缺点：栈太大    开销不确定

- Green Thread绿色线程

  特点：和OS的线程机制类似   有实际的线程和堆栈 也有上下文

  调度程序代替OS完成调度……

  缺点是还是有栈，栈可能需要增长……

- 回调   **保存指向一组命令的指针，在以后需要他的时候他会出来运行**

  运行时里面需要保存要回调的代码集合，以及一个回调过程/顺序 并为前后两段回调的闭包实现消息传递

  有点像**状态机** 状态不断变化 运行时管理各个状态的例程、

  缺点：每个程序都需要直到自己需要回调什么（后续到什么状态……

  - Promises 三种状态`Pending`   `Fulfilled` `Rejected`
  
- async/await

  使用开销为0（只有你能看到的代码才会有损耗

  

`async/.await` Rust内置语言特性   以同步方式编写异步代码

有`async`标记——》**转换为实现了`Future`特征的状态机**

在有`async`标记的函数里面使用`.await`可以等待另外一个异步调用的完成（只能推动async内层

这种方式不会阻塞当前线程……

```rust
// `block_on`会阻塞当前线程直到指定的`Future`执行完成，这种阻塞当前线程以等待任务完成的方式较为简单、粗暴，
// 好在其它运行时的执行器(executor)会提供更加复杂的行为，例如将多个`future`调度到同一个线程上执行。
use futures::executor::block_on;

async fn hello_world() {
    println!("hello, world!");
}

fn main() {
   	hello_world();  //直接调用是不会有任何结果的 因为此时这个函数还没有执行
    // 如何使用async声明的函数呢？  使用一个执行器
    let future = hello_world(); // 返回一个Future, 因此不会打印任何输出
    block_on(future); // 执行`Future`并等待其运行完成，此时"hello, world!"会被打印输出
}
```



## Rust Futures

`Future` 未来要完成的操作  **惰性**（只有被poll时才会运行  丢弃一个`Future`意味着阻止其未来被运行

每个异步任务分为三个阶段：轮询 等待 唤醒

```rust
trait Future {
    type Output;
    fn poll(
        // 首先值得注意的地方是，`self`的类型从`&mut self`变成了`Pin<&mut Self>`:
        self: Pin<&mut Self>,
        // 其次将`wake: fn()` 修改为 `cx: &mut Context<'_>`:
        // 意味着可以携带数据 而不只是一个简单的函数
        cx: &mut Context<'_>,
    ) -> Poll<Self::Output>;
}

```



`leaf-future` 实际正在等待的`future`

`Non-leaf-futures`   用`async`关键字创建的Future  可暂停的计算

由一系列的`leaf-future`组成 可将执行权交给调度程序 在之后再被唤醒



运行时（Runtimes

异步运行时分为

1. 执行器(The Executor)
2. reactor (The Reactor)

RUST不提供运行时 因此你需要主动选择运行时的实现（这点和其他语言不同

> 标准库只提供了`future trait`以及一些接口



`Waker` 唤醒器

和轮询以及执行器的逻辑松耦合……不与`Future`执行绑定

你接下来要干嘛 和我唤醒哪个无关





## 生成器

`Generator`  生成器

```rust
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
#[lang = "generator_state"]
pub enum GeneratorState<Y, R> {
    Yielded(Y),
    Complete(R),
}

#[lang = "generator"]
#[fundamental]
pub trait Generator<R = ()> {
    type Yield;
    type Return;
    fn resume(self: Pin<&mut Self>, arg: R) -> GeneratorState<Self::Yield, Self::Return>;
}

```

形式上和闭包类似  多了`yield`关键字

`yield`方法可以从生成器中返回   `resume`与之相对 可以从外界进入生成器

编译器会将生成器相关代码重写为状态机 



`async/await`的底层实现为`generator/yield`机制

为什么选择这样的实现？  主要了为了解决跨`yield`借用问题

结合`pin`机制将生成器的各种`gen()`锁住 从而能够保存其指针





## Pin

`Pin` 标记 固定内存位置 `!Unpin`特性标记即可实现

运行时零开销 编译期即可完成……

固定到栈上 不安全行为 需要用`unsafe`  

固定到堆上就没什么问题 长时间固定 比较安全

```rust
//如果你只能确定目标变量要被Pin固定一段时间 而后续不知道
//那最好是在生命周期到了之后手动释放Pin
fn main() {
   let mut test1 = Test::new("test1");
   let mut test1_pin = unsafe { Pin::new_unchecked(&mut test1) };
   Test::init(test1_pin.as_mut());

   drop(test1_pin);
   println!(r#"test1.b points to "test1": {:?}..."#, test1.b);

   let mut test2 = Test::new("test2");
   mem::swap(&mut test1, &mut test2);
   println!("... and now it points nowhere: {:?}", test1.b);
}

```

async提供的`Future`默认是`!unpin`的

有时候需要其是可移动的

```rust
use pin_utils::pin_mut; // `pin_utils` 可以在crates.io中找到

// 函数的参数是一个`Future`，但是要求该`Future`实现`Unpin`
fn execute_unpin_future(x: impl Future<Output = ()> + Unpin) { /* ... */ }

let fut = async { /* ... */ };
// 下面代码报错: 默认情况下，`fut` 实现的是`!Unpin`，并没有实现`Unpin`
// execute_unpin_future(fut);

// 使用`Box`进行固定
let fut = async { /* ... */ };
let fut = Box::pin(fut);
execute_unpin_future(fut); // OK

// 使用`pin_mut!`进行固定
let fut = async { /* ... */ };
pin_mut!(fut);
execute_unpin_future(fut); // OK

```

