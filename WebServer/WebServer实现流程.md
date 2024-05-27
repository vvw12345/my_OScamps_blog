# WebServer实现流程

## 基于线程池的WebServer实现
HTTP服务器 报文简要格式如下

响应报文也是基于这里的请求报文来编写

```markdown
Request: [
    "GET / HTTP/1.1",
    "Host: 127.0.0.1:8080",
    "Connection: keep-alive",
    "sec-ch-ua: \"Microsoft Edge\";v=\"125\", \"Chromium\";v=\"125\", \"Not.A/Brand\";v=\"24\"",
    "sec-ch-ua-mobile: ?0",
    "sec-ch-ua-platform: \"Windows\"",
    "Upgrade-Insecure-Requests: 1",
    "User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36 Edg/125.0.0.0",
    "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7",
    "Sec-Fetch-Site: none",
    "Sec-Fetch-Mode: navigate",
    "Sec-Fetch-User: ?1",
    "Sec-Fetch-Dest: document",
    "Accept-Encoding: gzip, deflate, br, zstd",
    "Accept-Language: zh-CN,zh;q=0.9,en;q=0.8,en-GB;q=0.7,en-US;q=0.6",
    "x-forwarded-for: 1.1.1.1",
]
```

### 线程池

线程池：提升并发处理能力的手段

最多可限制N个线程……

为每一个请求都创建一个线程吗？ 显然是不合理的……

应该是出现一个新的任务 在线程池中找到空闲线程 然后处理





## 基于异步的WebServer实现

到这里我在犹豫是调库还是自己弄一个

`tokio`在Rust是很成熟异步运行时

想了想自己对异步还是不太熟悉 所以接下来的实现会参照《200行讲透 Rust Future》

再将实现好的异步运行时提供给Web服务器……以供调用

主程序的逻辑与原先的基于线程池的实现区别是不大的……

还是监听消息并处理 只是这次是通过异步

