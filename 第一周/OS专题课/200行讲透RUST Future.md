# 200行讲透RUST Future

## 并发任务处理的主要思路

- OS线程    缺点：栈太大    开销不确定

- Green Thread绿色线程

  特点：和OS的线程机制类似   有实际的线程和堆栈 也有上下文

  调度程序代替OS完成调度……

  缺点是还是有栈，栈可能需要增长……

- 回调   **保存指向一组命令的指针，在以后需要他的时候他会出来运行**

  运行时里面需要保存要回调的代码集合，以及一个回调过程/顺序 并为前后两段回调的闭包实现消息传递

  有点像**状态机** 状态不断变化 运行时管理各个状态的例程、

  缺点：每个程序都需要直到自己需要回调什么（后续到什么状态……

  - Promises 三种状态`Pending`   `Fulfilled` `Rejected`



## Rust Futures

`Future` 未来要完成的操作

每个异步任务分为三个阶段：轮询 等待 唤醒



`leaf-future` 实际正在等待的`future`

`Non-leaf-futures`   用`async`关键字创建的Future  可暂停的计算

由一系列的`leaf-future`组成 可将执行权交给调度程序 在之后再被唤醒



运行时（Runtimes

异步运行时分为

1. 执行器(The Executor)
2. reactor (The Reactor)

RUST不提供运行时 因此你需要主动选择运行时的实现（这点和其他语言不同

> 标准库只提供了`future trait`以及一些接口