# my_OScamps_blog
Somethings about the Opening Operating System Camps by vvw12345
记录本人在OS训练营的一些经历和想法

## 时间线

### 第一周

#### 5.20

看了向勇老师的OS专题课程录屏并看了PPT

发现对RUST理解还是太不深刻 

所以重新看了一遍RUST圣经里面多线程编程部分 

最后看了200行讲透RUST Future的前两章



#### 5.21

看了RUST语言圣经里面对于异步编程的表述

然后是RUST Future的三四五章

……感觉很有难度 云里雾里的



#### 5.22

去医院了……

下午回来把圣经里面关于async/await部分再看了一遍 这次感觉比昨天好一点

之后看了一下200行 Future最后的部分 把一些特别疑难的点写了注释（主要是RUST语言的基础不牢固导致的

……个人感觉对200行 Future的示例代码缺乏一个整体的理解（可能是因为我没跑起来……明天计划跑起来看一下

最后花了差不多一个小时时间了解了一下四个选题方向

感觉好像都好难:cry:……



#### 5.24

- 今天看了embassy的中文文档 重点是`从裸机到异步RUST`部分

  给里面的代码添加了注释……现在看来其实是没必要（简单看懂就OK 尤其是PAC部分 细节过多了

  后面执行器`Executor`部分也看了一遍（不过没有做笔记  感觉其实还是对异步相关机制的讲解

  再后面就是`HAL`等部分了 说实话内容不多（但是很`莫名其妙`

  因为没有给出对应的代码 只是一个手册性质的介绍……总不能把`STM32`相关都去浏览一次吧

  把embassy的仓库拉了下来 看了一下代码比较多 `examples`哪里也没有很详细的说明

  就没执行了……

- 把200行讲透RUST Future的代码再看了一下 然后执行了一下

  只有两个`Future` 所以轮番被REACTOR唤醒

  ![Screenshot 2024-05-24 122528](D:\116\sigs\my_OScamps_blog\README.assets\Screenshot 2024-05-24 122528.png)

- 最后看了`Alien OS`的报告 主要思想是模块化操作系统

  复用社区实现好的内核模块来实现自己定制化的OS

后续可能对任务一比较感兴趣……可以进行英文文档的翻译，以及可以尝试一下在`Alien OS`或者一些类似的tiny OS上添加异步支持……



### 第二周

#### 5.26-5.27

两天一起记录一下吧

这两天选择去实现了一个异步的WebServer，总的来说是一个基于用户态开发的小程序

首先参照Rust圣经里面的内容 实现了一个基于线程池的WebServer

因为本科用C++做过 所以这里难度不大（主要是熟悉API

后面就是将线程池改为异步……

在做的过程中有纠结是调库还是不调库

`tokio`是相对成熟的Rust异步库 自己实现一个的话就是参照《200行 Future》的示例去做一个

后面选择了自己照着改……大致内容就这样 初步实现了一下

后续会对比二者的性能差异 给一个简要的总结



#### 5.28

先给昨天实现的tiny webserver加了点注释

今天找到了飓风内核的代码……

是一个异步内核的实现示例

看了一下文档 然后把源码拉下来

代码还是很不错的……个人预计需要些时间看……
