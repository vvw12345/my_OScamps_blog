# Rust笔记

## 基础入门

### 数据类型

1. 数据类型

   - 数值类型: 有符号整数 (`i8`, `i16`, `i32`, `i64`, `isize`)、 无符号整数 (`u8`, `u16`, `u32`, `u64`, `usize`) 、浮点数 (`f32`, `f64`)、以及有理数、复数
     - Nan表示未被定义的结果
     - debug模式检查整数溢出，release不会管
     - 浮点数不支持判等(eq操作未实现)
   - 字符串：字符串字面量和字符串切片 `&str`
   - 布尔类型： `true`和`false`，1个字节
   - 字符类型: 表示单个 **Unicode 字符**，存储为 **4 个字节**
   - 单元类型: 即 `()` ，其唯一的值也是 `()`

   一般来说不用显式声明，RUST编译器有变量推导

   比较逆天的话就不行了……

   ```rust
   let guess = "42".parse().expect("Not a number!");//推导不了
   
   //确定类型的三种方式
   // 编译器会进行自动推导，给予twenty i32的类型
   let twenty = 20;
   // 类型标注
   let twenty_one: i32 = 21;
   // 通过类型后缀的方式进行类型标注：22是i32类型
   let twenty_two = 22i32;
   ```

2. 序列

   生成**连续值**，只允许用于数字和字符类型（编译器可在编译期确定类型和判空）

   ```rust
   for i in 1..=5 {
       println!("{}",i);
   }
   
   for i in 'a'..='z' {
       println!("{}",i);
   }
   ```

3. 函数

   ![img](D:\116\sigs\my_OScamps_blog\一二阶段\Rust笔记.assets\v2-54b3a6d435d2482243edc4be9ab98153_1440w-1716128904543-3.png)

   ```rust
   fn add(i: i32, j: i32) -> i32 {
      i + j
    }
   ```

   - 特殊返回类型

     - 无返回值

       - 函数没有返回值，那么返回一个 `()`
       - 通过 `;` 结尾的语句返回一个 `()`

     - 发散函数：永不返回

       用`!`作为函数的返回类型

### 所有权和借用

1. C和RUST的内存管理差别

   ```c
   int* foo() {
       int a;          // 变量a的作用域开始
       a = 100;
       char *c = "xyz";   // 变量c的作用域开始
       return &a;
   }                   // 变量a和c的作用域结束
   //a是常数，被放在栈里，函数返回时出栈，a被回收，&a是悬空指针
   //c是字符串常量，在常量区，整个程序结束之后才会回收常量区
   ```

2. 所有权规则

   - Rust 中每一个值都被一个变量所拥有，该变量被称为值的所有者

   - 一个值同时只能被一个变量所拥有，或者说一个值只能拥有一个所有者

   - 当所有者(变量)离开作用域范围时，这个值将被丢弃(drop)

   ```rust
   let x = 5;
   let y = x;
   //浅拷贝，两个变量都依然有效
   
   let s1 = String::from("hello");
   let s2 = s1;
   //变量移动，默认是只copy指针，不会复制其实际内容
   //s1失效，s2接管那片内存空间
   
   let s1 = String::from("hello");
   let s2 = s1.clone();
   //你真想赋值的时候复制其内容，用clone()方法
   
   let x: &str = "hello, world";
   let y = x;
   //浅拷贝，"hello, world"是字符串字面量
   ```

   Copy特征：一个旧的变量在被赋值给其他变量后仍然可用，也就是赋值的过程即是拷贝的过程。**任何基本类型的组合可以 `Copy` ，不需要分配内存或某种形式资源的类型是可以 `Copy` 的**。

   - 所有整数类型，比如 `u32`
   - 布尔类型，`bool`，它的值是 `true` 和 `false`
   - 所有浮点数类型，比如 `f64`
   - 字符类型，`char`
   - 元组，当且仅当其包含的类型也都是 `Copy` 的时候。比如，`(i32, i32)` 是 `Copy` 的，但 `(i32, String)` 就不是
   - 不可变引用 `&T` ，例如[转移所有权](https://course.rs/basic/ownership/ownership.html#转移所有权)中的最后一个例子，**但是注意: 可变引用 `&mut T` 是不可以 Copy的**

3. 函数传值和返回——所有权的不断变化

   ```rust
   fn main() {
       let s1 = gives_ownership();         // gives_ownership 将返回值
                                           // 移给 s1
   
       let s2 = String::from("hello");     // s2 进入作用域
   
       let s3 = takes_and_gives_back(s2);  // s2 被移动到
                                           // takes_and_gives_back 中,
                                           // 它也将返回值移给 s3
   } // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，
     // 所以什么也不会发生。s1 移出作用域并被丢弃
   
   fn gives_ownership() -> String {             // gives_ownership 将返回值移动给
                                                // 调用它的函数
   
       let some_string = String::from("hello"); // some_string 进入作用域.
   
       some_string                              // 返回 some_string 并移出给调用的函数
   }
   
   // takes_and_gives_back 将传入字符串并返回该值
   fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域
   
       a_string  // 返回 a_string 并移出给调用的函数
   }
   ```

4. 引用

   ```rust
   fn main() {
       let s1 = String::from("hello");
   
       let len = calculate_length(&s1);//传入的是引用而不是所有权
   
       println!("The length of '{}' is {}.", s1, len);
   }
   
   fn calculate_length(s: &String) -> usize {
       s.len()//拿到的是引用，因此函数结束的时候不会释放所有权
   }
   
   ————————————————
   //引用默认不可变（就是你不能动你借用的东西的值）
   fn main() {
       let mut s = String::from("hello");//可变引用（可以修改借用的东西）
   
       change(&mut s);
   }
   
   fn change(some_string: &mut String) {
       some_string.push_str(", world");
   }
   
   ————————————————
   //在同一个作用域只可以存在一个可变引用（互斥锁懂我意思吧……）
   let mut s = String::from("hello");
   
   let r1 = &mut s;
   let r2 = &mut s;//r1的作用域还没寄，你怎么也搞个可变
   
   println!("{}, {}", r1, r2);
   
   
   ————————————————
   //可变和不可变引用不能同时存在
   let mut s = String::from("hello");
   
   let r1 = &s; // 没问题
   let r2 = &s; // 没问题
   let r3 = &mut s; // 大问题
   
   println!("{}, {}, and {}", r1, r2, r3);
   ```

   引用的作用域 `s` 从创建开始，一直持续到它最后一次使用的地方，这个跟变量的作用域有所不同，变量的作用域从创建持续到某一个花括号结束。

   ```rust
   fn main() {
      let mut s = String::from("hello");
   
       let r1 = &s;
       let r2 = &s;
       println!("{} and {}", r1, r2);
       // 新编译器中，r1,r2作用域在这里结束
   
       let r3 = &mut s;
       println!("{}", r3);
   } // 老编译器中，r1、r2、r3作用域在这里结束
     // 新编译器中，r3作用域在这里结束
   //Non-Lexical Lifetimes(NLL)特性：用于寻找到某个引用在`}`之前就不再被使用的位置
   ```

   悬垂引用在Rust是不会存在的，因为当你获取数据的引用后，编译器可以确保数据不会在引用结束前被释放，**要想释放数据，必须先停止其引用的使用**。

   ```rust
   fn main() {
       let reference_to_nothing = dangle();
   }
   
   fn dangle() -> &String {
       let s = String::from("hello");
   
       &s//悬垂引用，会报错
       //解决办法是直接返回s，也就是交出其所有权
   }
   ```

### 复合类型

#### 字符串和切片

1. **Rust 中的字符是 Unicode 类型，因此每个字符占据 4 个字节内存空间，但是在字符串中不一样，字符串是 UTF-8 编码，也就是字符串中的字符所占的字节数是变化的(1 - 4)**。

2. 为啥 `String` 可变，而字符串字面值 `str` 却不可以？

   就字符串字面值来说，我们在编译时就知道其内容，最终字面值文本被直接硬编码进可执行文件中，这使得字符串字面值快速且高效，这主要得益于字符串字面值的不可变性。不幸的是，我们不能为了获得这种性能，而把每一个在编译时大小未知的文本都放进内存中（你也做不到！），因为有的字符串是在程序运行的过程中动态生成的。

3. String和&str的转换

   ```rust
   //从&str生成String
   String::from("hello,world")
   "hello,world".to_string()
   
   //String到&str 取切片即可
   fn main() {
       let s = String::from("hello,world!");
       say_hello(&s);
       say_hello(&s[..]);
       say_hello(s.as_str());
   }
   
   fn say_hello(s: &str) {
       println!("{}",s);
   }
   ```

4. 字符串索引（Rust**不支持**）

   ```rust
   let s1 = String::from("hello");
   let hello = String::from("中国人");
   let h = s1[0];
   let h = hello[0];
   //不同字符的编码长度是不一样的，英文是1byte，中文是3byte，对特定单元的索引不一定有意义
   ```

   还有一个原因导致了 Rust 不允许去索引字符串：因为索引操作，我们总是期望它的性能表现是 O(1)，然而对于 `String` 类型来说，无法保证这一点，因为 Rust 可能需要从 0 开始去遍历字符串来定位合法的字符。

   字符串的区间切片Rust是**支持**的，但是必须谨慎

   ```rust
   let hello = "中国人";
   let s = &hello[0..2];
   ```

5. 常见字符串操作

   - 追加和插入

     ```rust
     //追加
     fn main() {
         let mut s = String::from("Hello ");//mut！
     
         s.push_str("rust");//追加字符串
     
         s.push('!');//追加单字符
     }
     
     //插入 insert需要插入位置和内容 位置越界会报错
     fn main() {
         let mut s = String::from("Hello rust!");//mut！
         s.insert(5, ',');
         s.insert_str(6, " I like");
     }
     ```

   - 替换

     ```rust
     //返回一个新的字符串，而不是操作原来的字符串！！！
     //replace  参数是：被替换内容，用来替换的内容
     fn main() {
         let string_replace = String::from("I like rust. Learning rust is my favorite!");
         let new_string_replace = string_replace.replace("rust", "RUST");
     }
     
     //replacen  和前面差不多，不过是替换n个匹配到的
     fn main() {
         let string_replace = "I like rust. Learning rust is my favorite!";
         let new_string_replacen = string_replace.replacen("rust", "RUST", 1);
         dbg!(new_string_replacen);
     }
     
     //方法是直接操作原来的字符串，不会返回新的字符串！！！
     //replace_range  替换特定范围
     fn main() {
         let mut string_replace_range = String::from("I like rust!");//mut！！！
         string_replace_range.replace_range(7..8, "R");
     }
     ```

   - 删除

     ```rust
     //直接操作原来的字符串  mut！！！
     //pop  删除并返回最后一个字符 由于不确保存在，返回的是Option()类型 需要具体考察
     fn main() {
         let mut string_pop = String::from("rust pop 中文!");
         let p1 = string_pop.pop();
         let p2 = string_pop.pop();
     }
     
     //remove 删除指定位置的一个字符  注意给的索引要合法（表示字符的起始位置）
     fn main() {
         let mut string_remove = String::from("测试remove方法");
         println!(
             "string_remove 占 {} 个字节",
             std::mem::size_of_val(string_remove.as_str())
         );
         // 删除第一个汉字
         string_remove.remove(0);
         // 下面代码会发生错误
         // string_remove.remove(1);
         // 直接删除第二个汉字
         // string_remove.remove(3);
         dbg!(string_remove);
     }
     
     //truncate 从当前位置直接删除到结尾 注意给的索引
     fn main() {
         let mut string_truncate = String::from("测试truncate");
         string_truncate.truncate(3);
     }
     
     //clear 清空
     fn main() {
         let mut string_clear = String::from("string clear");
         string_clear.clear();
         dbg!(string_clear);
     }
     ```

   - 连接

     ```rust
     //+或+=   +右边的必须是切片引用类型
     //返回一个新的字符串，所以变量声明可以不需要 mut 关键字修饰
     fn main() {
         let string_append = String::from("hello ");
         let string_rust = String::from("rust");
         // &string_rust会自动解引用为&str
         let result = string_append + &string_rust;
         let mut result = result + "!"; // `result + "!"` 中的 `result` 是不可变的
         result += "!!!";
     
         println!("连接字符串 + -> {}", result);
     }
     
     //format!() 格式化输出
     fn main() {
         let s1 = "hello";
         let s2 = String::from("rust");
         let s = format!("{} {}!", s1, s2);
         println!("{}", s);
     }
     ```

#### 元组

```rust
//模式匹配解构元组
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
}

//用.访问元组
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```

#### 结构体

1. 结构体语法

   ```rust
   //创建
   struct User {
       active: bool,
       username: String,
       email: String,
       sign_in_count: u64,
   }
   
   //初始化  每个字段都要初始化
       let user1 = User {
           email: String::from("someone@example.com"),
           username: String::from("someusername123"),
           active: true,
           sign_in_count: 1,
       };
   
   //通过.来访问结构体内部字段
       let mut user1 = User {  //要改的话还是要mut
           email: String::from("someone@example.com"),
           username: String::from("someusername123"),
           active: true,
           sign_in_count: 1,
       };
   
       user1.email = String::from("anotheremail@example.com");
   
   //当函数参数和结构体字段名称一样时，可以简写
   fn build_user(email: String, username: String) -> User {
       User {
           email,
           username,//缩略的初始化
           active: true,
           sign_in_count: 1,
       }
   }
   
   //更新
     let user2 = User {
           email: String::from("another@example.com"),
           ..user1  //未显式声明的字段都会从user1中获取   不过..user1只可以写在末尾
       };//也就是说你要赋值的写在前面
   
   //更新过程可能会有某些字段发生了所有权的转移，不会影响其他字段的访问
   let user1 = User {
       email: String::from("someone@example.com"),
       username: String::from("someusername123"),
       active: true,
       sign_in_count: 1,
   };
   let user2 = User {
       active: user1.active,
       username: user1.username,
       email: String::from("another@example.com"),
       sign_in_count: user1.sign_in_count,
   };
   println!("{}", user1.active);
   // 下面这行会报错
   println!("{:?}", user1);
   
   ```

2. 元组结构体

   为整个结构体提供名称，而字段不需要

   ```rust
   struct Color(i32, i32, i32);
   struct Point(i32, i32, i32);
   
   let black = Color(0, 0, 0);
   let origin = Point(0, 0, 0);
   ```

3. 单元结构体：没有任何字段和属性的结构体

#### 枚举

1. 任何数据类型都可以放到枚举中

   ```rust
   enum PokerCard {
       Clubs(u8),
       Spades(u8),
       Diamonds(char),//定义枚举成员时关联数据
       Hearts(char),
   }
   
   fn main() {
      let c1 = PokerCard::Spades(5);
      let c2 = PokerCard::Diamonds('A');
   }
   ```

2. 枚举和结构体的对比

   ```rust
   //使用枚举来定义这些消息
   enum Message {
       Quit,
       Move { x: i32, y: i32 },
       Write(String),
       ChangeColor(i32, i32, i32),
   }
   
   fn main() {
       let m1 = Message::Quit;
       let m2 = Message::Move{x:1,y:1};
       let m3 = Message::ChangeColor(255,255,0);
   }
   
   //使用结构体来定义这些消息
   struct QuitMessage; // 单元结构体
   struct MoveMessage {
       x: i32,
       y: i32,
   }
   struct WriteMessage(String); // 元组结构体
   struct ChangeColorMessage(i32, i32, i32); // 元组结构体
   
   //由于每个结构体都有自己的类型，因此我们无法在需要同一类型的地方进行使用，例如某个函数它的功能是接受消息并进行发送，那么用枚举的方式，就可以接收不同的消息，但是用结构体，该函数无法接受 4 个不同的结构体作为参数。
   ```

3. 取代NULL的方式——Option()枚举

   ```rust
   //Option()枚举定义
   enum Option<T> {
       Some(T), //T可以是任何类型
       None,
   }
   
   //示例
   ——————————————
   
   let some_number = Some(5);
   let some_string = Some("a string");
   
   let absent_number: Option<i32> = None;
   //当有个None值时，你需要告诉编译器T的类型，因为编译器无法通过None来推断本来应该是什么
   ```

4. Option()枚举的好处

   ```rust
   let x: i8 = 5;
   let y: Option<i8> = Some(5);
   
   let sum = x + y;//报错！Option(i8)和i8并不是同一种类型
   ```

   当在 Rust 中拥有一个像 `i8` 这样类型的值时，编译器确保它总是有一个有效的值，我们可以放心使用而无需做空值检查。只有当使用 `Option<i8>`（或者任何用到的类型）的时候才需要担心可能没有值，而编译器会确保我们在使用值之前处理了为空的情况。

   换句话说，在对 `Option<T>` 进行 `T` 的运算之前必须将其转换为 `T`。通常这能帮助我们捕获到空值最常见的问题之一：期望某值不为空但实际上为空的情况。

5. match表达式可以用于处理枚举

   ```rust
   fn plus_one(x: Option<i32>) -> Option<i32> {
       match x {
           None => None,
           Some(i) => Some(i + 1),
       }//如果接收到Some(i)类型，将其中的变量绑定到i上，计算i+1，再将其用Some()包裹
   }
   
   let five = Some(5);
   let six = plus_one(five);
   let none = plus_one(None);
   ```

#### 数组

1. 创建

   ```rust
   //RUST的数组是定长的，被存储在栈上
   //变长的动态数组被存储在堆上
   //数组的长度也是类型的一部分
   let a: [i32; 5] = [1, 2, 3, 4, 5];
   
   //声明多个重复值
   let a = [3; 5];
   
   //非基础类型数组的创建
   
   //这样子写会报错，本质还是因为string不能浅拷贝
   let array = [String::from("rust is good!"); 8];
   
   //这样子写可以，但是很难看
   let array = [String::from("rust is good!"),String::from("rust is good!"),String::from("rust is good!")];
   
   //遇到非基本类型数组 调用std::array::from_fn
   let array: [String; 8] = std::array::from_fn(|_i| String::from("rust is good!"));
   ```

2. 支持索引访问，**如果越界会崩溃**

### 流程控制

```rust
fn main() {
    for i in 1..=5 {
        println!("{}", i);
    }
}

//如果想在循环中获取元素的索引，使用.iter()方法获得迭代器
fn main() {
    let a = [4, 3, 2, 1];
    // `.iter()` 方法把 `a` 数组变成一个迭代器
    for (i, v) in a.iter().enumerate() {
        println!("第{}个元素是{}", i + 1, v);
    }
}
```

| 使用方法                      | 等价使用方式                                      | 所有权     |
| ----------------------------- | ------------------------------------------------- | ---------- |
| `for item in collection`      | `for item in IntoIterator::into_iter(collection)` | 转移所有权 |
| `for item in &collection`     | `for item in collection.iter()`                   | 不可变借用 |
| `for item in &mut collection` | `for item in collection.iter_mut()`               | 可变借用   |

```rust
// 第一种
let collection = [1, 2, 3, 4, 5];
for i in 0..collection.len() {
  let item = collection[i];
  // ...
}

// 第二种
for item in collection {

}


//while循环
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index = index + 1;
    }
}//用while循环来实现和第一种for循环是一样的
```

第一种方式是循环索引，然后通过索引下标去访问集合，第二种方式是直接循环集合中的元素，优劣如下：

- **性能**：第一种使用方式中 `collection[index]` 的索引访问，会因为边界检查(Bounds Checking)导致运行时的性能损耗 —— Rust 会检查并确认 `index` 是否落在集合内，但是第二种直接迭代的方式就不会触发这种检查，因为编译器会在编译时就完成分析并证明这种访问是合法的
- **安全**：第一种方式里对 `collection` 的索引访问是非连续的，存在一定可能性在两次访问之间，`collection` 发生了变化，导致脏数据产生。而第二种直接迭代的方式是连续访问，因此不存在这种风险( 由于所有权限制，在访问过程中，数据并不会发生变化)。



loop：简单的无限循环，需要搭配break跳出

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;//break可以单独使用直接跳出，也可以带一个值返回（类似return）
        }
    };

    println!("The result is {}", result);
}
```



### 模式匹配

#### match和if let

1. 匹配

   ```rust
   enum Coin {
       Penny,
       Nickel,
       Dime,
       Quarter,
   }
   
   fn value_in_cents(coin: Coin) -> u8 {
       match coin {
           Coin::Penny =>  {
               println!("Lucky penny!");
               1
           },
           Coin::Nickel => 5,
           Coin::Dime => 10,
           Coin::Quarter => 25,
       }
   }
   //match匹配需要穷尽所有的可能，用_表示没有列出的其他可能性(如果没有穷尽可能性的话会报错)
   //match的每一个分支都必须是一个表达式,且所有分支的表达式最终返回值的类型必须相同
   ```

2. 模式绑定

   ```rust
   #[derive(Debug)]
   enum UsState {
       Alabama,
       Alaska,
       // --snip--
   }
   
   enum Coin {
       Penny,
       Nickel,
       Dime,
       Quarter(UsState), // 25美分硬币
   }
   
   fn value_in_cents(coin: Coin) -> u8 {
       match coin {
           Coin::Penny => 1,
           Coin::Nickel => 5,
           Coin::Dime => 10,
           Coin::Quarter(state) => {//这里将枚举类别Coin中的UsState值绑定给state变量
               println!("State quarter from {:?}!", state);
               25
           },
       }
   }
   ```

3. if let匹配

   当我们只关注某个特定的值的匹配情况时，可以使用if let匹配代替match

   ```rust
   let v = Some(3u8);
   match v {
       Some(3) => println!("three"),
       _ => (),
   }
   
   //if let匹配
   if let Some(3) = v {
       println!("three");
   }
   ```

4. matches!()宏

   将表达式和模式进行匹配，返回True或者False

   ```rust
   enum MyEnum {
       Foo,
       Bar
   }
   
   fn main() {
       let v = vec![MyEnum::Foo,MyEnum::Bar,MyEnum::Foo];
   }
   //对v进行过滤，只保留类型为MyEnum::Foo的元素
   v.iter().filter(|x| matches!(x, MyEnum::Foo));
   
   //更多例子
   let foo = 'f';
   assert!(matches!(foo, 'A'..='Z' | 'a'..='z'));
   
   let bar = Some(4);
   assert!(matches!(bar, Some(x) if x > 2));
   ```

5. match和if let匹配导致的变量遮蔽

   ​	尽量不要使用同名变量

   ```rust
   fn main() {
      let age = Some(30);
      println!("在匹配前，age是{:?}",age);
      if let Some(age) = age {
          println!("匹配出来的age是{}",age);
      }
   
      println!("在匹配后，age是{:?}",age);
   }
   
   fn main() {
      let age = Some(30);
      println!("在匹配前，age是{:?}",age);
      match age {
          Some(age) =>  println!("匹配出来的age是{}",age),
          _ => ()
      }
      println!("在匹配后，age是{:?}",age);
   }
   ```

#### 一些模式适用场景

1. while let 只要匹配就会一直循环下去

   ```rust
   // Vec是动态数组
   let mut stack = Vec::new();
   
   // 向数组尾部插入元素
   stack.push(1);
   stack.push(2);
   stack.push(3);
   
   // stack.pop从数组尾部弹出元素
   while let Some(top) = stack.pop() {
       println!("{}", top);
   }
   ```

2. let和if let

   ```rust
   let Some(x) = some_option_value;//报错，有可能是None
   //let，match，for都需要完全匹配(不可驳匹配)
   
   if let Some(x) = some_option_value {
       println!("{}", x);
   }//通过，只要有值的情况，其余情况忽略(可驳模式匹配)
   ```

#### 全模式列表

1. 用序列语法`..=`匹配区间内的值（还是只能用于数字和字符）

   ```rust
   let x = 5;
   
   match x {
       1..=5 => println!("one through five"),
       _ => println!("something else"),
   }
   
   ```

2. 使用模式忽略值

   ```rust
   //忽略函数变量
   fn foo(_: i32, y: i32) {
       println!("This code only uses the y parameter: {}", y);
   }
   
   fn main() {
       foo(3, 4);
   }
   ```

   用`_`忽略值和用`_s`的区别

   ```rust
   let s = Some(String::from("Hello!"));
   
   if let Some(_s) = s {
       println!("found a string");
   }
   
   println!("{:?}", s);//会报错，因为s的所有权已经转移给_s了
   
   ——————————————————————————
   
   let s = Some(String::from("Hello!"));
   
   if let Some(_) = s {
       println!("found a string");
   }
   
   println!("{:?}", s);//使用下划线本身是不会绑定值的
   ```

3. 使用`..`忽略多个值需要保证没有歧义

   ```rust
   fn main() {
       let numbers = (2, 4, 8, 16, 32);
   
       match numbers {
           (.., second, ..) => {
               println!("Some numbers: {}", second)
           },
       }
   }//报错，编译器无法理解second具体指哪个
   ```

4. 匹配守卫——为匹配提供额外条件

   ```rust
   fn main() {
       let x = Some(5);
       let y = 10;
   
       match x {
           Some(50) => println!("Got 50"),
           Some(n) if n == y => println!("Matched, n = {}", n),
           //通过匹配守卫，使得在匹配中也可以正常的使用外部变量，而不用担心变量遮蔽的问题
           _ => println!("Default case, x = {:?}", x),
       }
   
       println!("at the end: x = {:?}, y = {}", x, y);
   }
   
   ——————————————————
   //匹配守卫的优先级：会作用于所有的匹配项
   let x = 4;
   let y = false;
   
   match x {
       4 | 5 | 6 if y => println!("yes"),
       _ => println!("no"),
   }
   
   ```

5. @绑定——提供在限定范围条件下，在分支代码内部使用变量的能力

   ```rust
   enum Message {
       Hello { id: i32 },
   }
   
   let msg = Message::Hello { id: 5 };
   
   match msg {
       Message::Hello { id: id_variable @ 3..=7 } => {
           println!("Found an id in range: {}", id_variable)
       },//@变量绑定，限定范围且绑定变量
       Message::Hello { id: 10..=12 } => {
           println!("Found an id in another range")
       },//限定了范围，但是这样子只会匹配，而id这个量用不了
       Message::Hello { id } => {
           println!("Found some other id: {}", id)
       },//可以匹配并绑定到id上，但是这样子限制不了范围
   }
   
   
   ————————————————
   //绑定的同时对变量结构
   #[derive(Debug)]
   struct Point {
       x: i32,
       y: i32,
   }
   
   fn main() {
       // 绑定新变量 `p`，同时对 `Point` 进行解构
       let p @ Point {x: px, y: py } = Point {x: 10, y: 23};
       println!("x: {}, y: {}", px, py);
       println!("{:?}", p);
   
       let point = Point {x: 10, y: 5};
       if let p @ Point {x: 10, y} = point {
           println!("x is 10 and y is {} in {:?}", y, p);
       } else {
           println!("x was not 10 :(");
       }
   }
   ```

   

### 方法Method

1. 定义和初始化

   ```rust
   struct Circle {
       x: f64,
       y: f64,
       radius: f64,
   }
   
   impl Circle {
       // new是Circle的关联函数，因为它的第一个参数不是self，且new并不是关键字
       // 这种方法往往用于初始化当前结构体的实例
       fn new(x: f64, y: f64, radius: f64) -> Circle {
           Circle {
               x: x,
               y: y,
               radius: radius,
           }
       }
   
       // Circle的方法，&self表示借用当前的Circle结构体
       fn area(&self) -> f64 {
           std::f64::consts::PI * (self.radius * self.radius)
       }
   }
   ```

   ​	这种定义在 `impl` 中且没有 `self` 的函数被称之为**关联函数**： 因为它没有 `self`，不能用 `f.read()` 的形式调用，因此它是一个函数而不是方法，它又在 `impl` 中，与结构体紧密关联，因此称为关联函数。

   ​	因为是函数，所以不能用 `.` 的方式来调用，我们需要用 `::` 来调用，例如 `let sq = Rectangle::new(3, 3);`。这个方法位于结构体的命名空间中：`::` 语法用于关联函数和模块创建的命名空间。

   ![img](D:\116\sigs\my_OScamps_blog\一二阶段\Rust笔记.assets\v2-0d848e960f3279999eab4b1317f6538e_1440w.png)

   其他的语言往往将类型和方法一起定义，而Rust对这两者的定义是分开的。

2. self和被实例化类型的关系

   ```rust
   #[derive(Debug)]
   struct Rectangle {
       width: u32,
       height: u32,
   }
   
   impl Rectangle {//方法名称可以和结构体的名称相同
       fn area(&self) -> u32 {
           self.width * self.height
       }
       //self 表示 Rectangle 的所有权转移到该方法中，这种形式用的较少
       //&self 表示该方法对 Rectangle 的不可变借用
       //&mut self 表示可变借用
   
   }
   
   fn main() {
       let rect1 = Rectangle { width: 30, height: 50 };
   
       println!(
           "The area of the rectangle is {} square pixels.",
           rect1.area()
       );
   }
   ```

3. 方法和字段同名的好处

   ```rust
   pub struct Rectangle {
       width: u32,
       height: u32,
   }
   
   impl Rectangle {
       pub fn new(width: u32, height: u32) -> Self {
           Rectangle { width, height }
       }
       pub fn width(&self) -> u32 {
           return self.width;
       }
   }
   
   fn main() {
       let rect1 = Rectangle::new(30, 50);
   
       println!("{}", rect1.width());
   }
   ```

   ​	方法和字段同名有助于我们实现访问器，我们可以将`width`和`height`设置为私有属性，而通过`pub`关键字将`Rectangle`结构体对应的`new`方法和`width`方法设置为公有方法，这样子用户可以通过`rect1.width()`方法访问到宽度的数据，却无法直接使用`rect1.width`来访问。

4. Rust中用自动引用/解引用机制代替了C/C++的->运算符

   ​	在 C/C++ 语言中，有两个不同的运算符来调用方法：`.` 直接在对象上调用方法，而 `->` 在一个对象的指针上调用方法，这时需要先解引用指针。换句话说，如果 `object` 是一个指针，那么 `object->something()` 和 `(*object).something()` 是一样的。

   ​	Rust 并没有一个与 `->` 等效的运算符；相反，Rust 有一个叫 **自动引用和解引用**的功能。方法调用是 Rust 中少数几个拥有这种行为的地方。

   ​	他是这样工作的：当使用 `object.something()` 调用方法时，Rust 会自动为 `object` 添加 `&`、`&mut` 或 `*` 以便使 `object` 与方法签名匹配。也就是说，这些代码是等价的：

   ```rust
   p1.distance(&p2);
   (&p1).distance(&p2);
   ```

   ​	第一行看起来简洁的多。这种自动引用的行为之所以有效，是因为方法有一个明确的接收者———— `self` 的类型。在给出接收者和方法名的前提下，Rust 可以明确地计算出方法是仅仅读取（`&self`），做出修改（`&mut self`）或者是获取所有权（`self`）。事实上，Rust 对方法接收者的隐式借用让所有权在实践中更友好。

### 泛型和特征

#### 泛型

1. 代替值的泛型，而不是针对类型的泛型

   ```rust
   //这段代码会报错，因为不同长度的数组在Rust中是不同的类型
   fn display_array(arr: [i32; 3]) {
       println!("{:?}", arr);
   }
   fn main() {
       let arr: [i32; 3] = [1, 2, 3];
       display_array(arr);
   
       let arr: [i32; 2] = [1, 2];
       display_array(arr);
   }
   
   //用切片的方式打印任意长度的数组，同时用泛型指代不同的类型
   fn display_array<T: std::fmt::Debug>(arr: &[T]) {
       println!("{:?}", arr);
   }
   fn main() {
       let arr: [i32; 3] = [1, 2, 3];
       display_array(&arr);
   
       let arr: [i32; 2] = [1, 2];
       display_array(&arr);
   }
   
   //切片是一种引用，但是有的场景不允许我们使用引用，此时通过const泛型指代不同的长度
   fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
       println!("{:?}", arr);
   }
   fn main() {
       let arr: [i32; 3] = [1, 2, 3];
       display_array(arr);
   
       let arr: [i32; 2] = [1, 2];
       display_array(arr);
   }
   ```

2. 泛型的性能

   编译器完成**单态化**的过程，增加了编译的繁琐程度，也让编译后的文件更大

   会对每一个具体用到的类型都生成一份代码

   ```rust
   //程序编写
   let integer = Some(5);
   let float = Some(5.0);
   
   //编译后
   enum Option_i32 {
       Some(i32),
       None,
   }
   
   enum Option_f64 {
       Some(f64),
       None,
   }
   
   fn main() {
       let integer = Option_i32::Some(5);
       let float = Option_f64::Some(5.0);
   }
   ```

#### 特征

​	一组可以被共享的行为，只要满足了特征，就可以做以下的行为。

1. 定义特征

   ​	只管定义，而往往不会提供具体的实现

   ​	谁满足这个特征，谁来实现具体的方法

   ```rust
   pub trait Summary {
       fn summarize(&self) -> String;//以;结尾 只提供定义
   }
   
   pub trait Summary {
       fn summarize(&self) -> String { //也可以给一个默认实现
           String::from("(Read more...)")
       }//可以调用，也可以重载
   }
   ```

   ​	**默认实现允许调用相同特征中的其他方法，哪怕这些方法没有默认实现。**如此，特征可以提供很多有用的功能而只需要实现指定的一小部分内容。例如，我们可以定义 `Summary` 特征，使其具有一个需要实现的 `summarize_author` 方法，然后定义一个 `summarize` 方法，此方法的默认实现调用 `summarize_author` 方法：

   ```rust
   pub trait Summary {
       fn summarize_author(&self) -> String;//让实现Summary特征的类型具体实现吧
   
       fn summarize(&self) -> String {
           format!("(Read more from {}...)", self.summarize_author())
       }
   }
   ```

2. 实现特征

   ```rust
   pub trait Summary {
       fn summarize(&self) -> String;
   }
   pub struct Post {
       pub title: String, // 标题
       pub author: String, // 作者
       pub content: String, // 内容
   }
   
   impl Summary for Post {//为Post实现Summary特征
       fn summarize(&self) -> String {
           format!("文章{}, 作者是{}", self.title, self.author)
       }
   }
   
   pub struct Weibo {
       pub username: String,
       pub content: String
   }
   
   impl Summary for Weibo {
       fn summarize(&self) -> String {
           format!("{}发表了微博{}", self.username, self.content)
       }
   }
   ```

3. 孤儿规则——特征定义和实现的位置关系

   ​	关于特征实现与定义的位置，有一条非常重要的原则：**如果你想要为类型** `A` **实现特征** `T`**，那么** `A` **或者** `T` **至少有一个是在当前作用域中定义的！** 例如我们可以为上面的 `Post` 类型实现标准库中的 `Display` 特征，这是因为 `Post` 类型定义在当前的作用域中。同时，我们也可以在当前包中为 `String` 类型实现 `Summary` 特征，因为 `Summary` 定义在当前作用域中。

   ​	但是你无法在当前作用域中，为 `String` 类型实现 `Display` 特征，因为它们俩都定义在标准库中，其定义所在的位置都不在当前作用域，跟你半毛钱关系都没有，看看就行了。

4. 使用特征作为函数的参数

   ```rust
   pub fn notify(item: &impl Summary) {//实现了特征Summary的item参数
       println!("Breaking news! {}", item.summarize());//可以调用特征对应的方法
   }
   ```

5. 特征约束

   ```rust
   //接收两个实现了Summary特征的参数，但是不能保证这两个参数的类型相同
   pub fn notify(item1: &impl Summary, item2: &impl Summary) {}
   
   //用泛型T指代
   //T:Summary要求其实现了特征Summary
   pub fn notify<T: Summary>(item1: &T, item2: &T) {}
   
   //多重约束
   //这里T被要求同时实现两个特征才行
   pub fn notify<T: Summary + Display>(item: &T) {}
   
   //Where约束，主要是用于简化函数的签名，将特征约束写在别处
   fn some_function<T, U>(t: &T, u: &U) -> i32
       where T: Display + Clone,
             U: Clone + Debug
   {}
   ```

6. 函数返回值中的impl Trait

   ```rust
   fn returns_summarizable() -> impl Summary {
       //返回一个实现了Summary特征的类型，具体是什么类型不知道
       Weibo {
           username: String::from("sunface"),
           content: String::from(
               "m1 max太厉害了，电脑再也不会卡",
           )
       }
   }
   ```

   ​	这种 `impl Trait` 形式的返回值，在一种场景下非常非常有用，那就是返回的真实类型非常复杂，你不知道该怎么声明时(毕竟 Rust 要求你必须标出所有的类型)，此时就可以用 `impl Trait` 的方式简单返回。

7. derive派生特征

   ​	在本书中，形如 `#[derive(Debug)]` 的代码已经出现了很多次，这种是一种特征派生语法，被 `derive` 标记的对象会自动实现对应的默认特征代码，继承相应的功能。

   ​	例如 `Debug` 特征，它有一套自动实现的默认代码，当你给一个结构体标记后，就可以使用 `println!("{:?}", s)` 的形式打印该结构体的对象。

   ​	再如 `Copy` 特征，它也有一套自动实现的默认代码，当标记到一个类型上时，可以让这个类型自动实现 `Copy` 特征，进而可以调用 `copy` 方法，进行自我复制。

   ​	总之，`derive` 派生出来的是 Rust 默认给我们提供的特征，在开发过程中极大的简化了自己手动实现相应特征的需求，当然，如果你有特殊的需求，还可以自己手动重载该实现。

#### 特征对象

​	指向了所有实现了某特征的对象，二者之间存在映射关系，可以通过特征对象找到该对象具体的实现方法。

1. 可以通过 `&` 引用或者 `Box<T>` 智能指针的方式来创建特征对象

   ```rust
   trait Draw {
       fn draw(&self) -> String;
   }
   
   impl Draw for u8 {
       fn draw(&self) -> String {
           format!("u8: {}", *self)
       }
   }
   
   impl Draw for f64 {
       fn draw(&self) -> String {
           format!("f64: {}", *self)
       }
   }
   
   // 若 T 实现了 Draw 特征， 则调用该函数时传入的 Box<T> 可以被隐式转换成函数参数签名中的 Box<dyn Draw>
   fn draw1(x: Box<dyn Draw>) {
       // 由于实现了 Deref 特征，Box 智能指针会自动解引用为它所包裹的值，然后调用该值对应的类型上定义的 `draw` 方法
       x.draw();
   }
   
   fn draw2(x: &dyn Draw) {
       x.draw();
   }
   
   fn main() {
       let x = 1.1f64;
       // do_something(&x);
       let y = 8u8;
   
       // x 和 y 的类型 T 都实现了 `Draw` 特征，因为 Box<T> 可以在函数调用时隐式地被转换为特征对象 Box<dyn Draw> 
       // 基于 x 的值创建一个 Box<f64> 类型的智能指针，指针指向的数据被放置在了堆上
       draw1(Box::new(x));
       // 基于 y 的值创建一个 Box<u8> 类型的智能指针
       draw1(Box::new(y));
       draw2(&x);
       draw2(&y);
   }
   ```

   - `draw1` 函数的参数是 `Box<dyn Draw>` 形式的特征对象，该特征对象是通过 `Box::new(x)` 的方式创建的
   - `draw2` 函数的参数是 `&dyn Draw` 形式的特征对象，该特征对象是通过 `&x` 的方式创建的
   - `dyn` 关键字只用在特征对象的类型声明上，在创建时无需使用 `dyn`

   可以通过特征对象来代表具体的泛型。

2. 使用泛型的实现和特征对象的对比

   ```rust
   pub struct Screen<T: Draw> {
       pub components: Vec<T>,
   }
   
   impl<T> Screen<T>
       where T: Draw {
       pub fn run(&self) {
           for component in self.components.iter() {
               component.draw();
           }
       }
   }
   
   ```

   ​	上面的 `Screen` 的列表中，存储了类型为 `T` 的元素，然后在 `Screen` 中使用特征约束让 `T` 实现了 `Draw` 特征，进而可以调用 `draw` 方法。

   ​	但是这种写法限制了 `Screen` 实例的 `Vec<T>` 中的每个元素必须是 `Button` 类型或者全是 `SelectBox` 类型。如果只需要同质（相同类型）集合，更倾向于采用泛型+特征约束这种写法，因其实现更清晰，且性能更好(特征对象，需要在运行时从 `vtable` 动态查找需要调用的方法)。

3. 特征对象的限制

   **不是所有特征都能拥有特征对象，只有对象安全的特征才行。**当一个特征的所有方法都有如下属性时，它的对象才是安全的：

   - 方法的返回类型不能是 `Self`
   - 方法没有任何泛型参数

   对象安全对于特征对象是必须的，因为一旦有了特征对象，就不再需要知道实现该特征的具体类型是什么了。如果特征方法返回了具体的 `Self` 类型，但是特征对象忘记了其真正的类型，那这个 `Self` 就非常尴尬，因为没人知道它是谁了。但是对于泛型类型参数来说，当使用特征时其会放入具体的类型参数：此具体类型变成了实现该特征的类型的一部分。而当使用特征对象时其具体类型被抹去了，故而无从得知放入泛型参数类型到底是什么。

   标准库中的 `Clone` 特征就不符合对象安全的要求：

   ```rust
   pub trait Clone {
       fn clone(&self) -> Self;
   }
   ```

   因为它的其中一个方法，返回了 `Self` 类型，因此它是对象不安全的。

4. 特征对象的动态分发

   ​	静态分发：编译器会为每一个泛型参数对应的具体类型生成一份代码

   ​	动态分发：直到运行时，才能确定需要调用什么方法。编译器无法知晓所有可能用于特征对象代码的类型，所以它也不知道应该调用哪个类型的哪个方法实现。

   ![img](D:\116\sigs\my_OScamps_blog\一二阶段\Rust笔记.assets\v2-b771fe4cfc6ebd63d9aff42840eb8e67_1440w.jpg)

   ​	

   - **特征对象大小不固定**：这是因为，对于特征 `Draw`，类型 `Button` 可以实现特征 `Draw`，类型 `SelectBox` 也可以实现特征 `Draw`，因此特征没有固定大小
   - 几乎总是使用特征对象的引用方式，如`&dyn Draw`和`Box<dyn Draw>`
     - 虽然特征对象没有固定大小，但它的引用类型的大小是固定的，它由两个指针组成（`ptr` 和 `vptr`），因此占用两个指针大小
     - 一个指针 `ptr` 指向实现了特征 `Draw` 的具体类型的实例，也就是当作特征 `Draw` 来用的类型的实例，比如类型 `Button` 的实例、类型 `SelectBox` 的实例
     - 另一个指针 `vptr` 指向一个虚表 `vtable`，`vtable` 中保存了类型 `Button` 或类型 `SelectBox` 的实例对于可以调用的实现于特征 `Draw` 的方法。当调用方法时，直接从 `vtable` 中找到方法并调用。之所以要使用一个 `vtable` 来保存各实例的方法，是因为实现了特征 `Draw` 的类型有多种，这些类型拥有的方法各不相同，当将这些类型的实例都当作特征 `Draw` 来使用时(此时，它们全都看作是特征 `Draw` 类型的实例)，有必要区分这些实例各自有哪些方法可调用

   简而言之，当类型 `Button` 实现了特征 `Draw` 时，类型 `Button` 的实例对象 `btn` 可以当作特征 `Draw` 的特征对象类型来使用，`btn` 中保存了作为特征对象的数据指针（指向类型 `Button` 的实例数据）和行为指针（指向 `vtable`）。

   一定要注意，此时的 `btn` 是 `Draw` 的特征对象的实例，而不再是具体类型 `Button` 的实例，而且 `btn` 的 `vtable` 只包含了实现自特征 `Draw` 的那些方法（比如 `draw`），因此 `btn` 只能调用实现于特征 `Draw` 的 `draw` 方法，而不能调用类型 `Button` 本身实现的方法和类型 `Button` 实现于其他特征的方法。**也就是说，`btn` 是哪个特征对象的实例，它的 `vtable` 中就包含了该特征的方法。**

#### 特征进阶内容

1. 关联类型

   在特征定义的语句块中，声明一个自定义类型，这样就可以在特征中使用这个类型。

   ```rust
   pub trait Iterator {
       type Item;
   
       fn next(&mut self) -> Option<Self::Item>;
   }
   ```





### 集合类型

#### 动态数组vector

1. 创建

   ```rust
   //调用关联函数new()
   let v: Vec<i32> = Vec::new();//需要声明具体类型
   
   let mut v = Vec::new();
   v.push(1);//给了一个元素，所以不用声明类型了，可以推导出来
   
   //vec!()宏创建  好处在于创建的同时可以初始化
   let v = vec![1, 2, 3];
   ```

2. 更新   mut可变才可以

   ```rust
   let mut v = Vec::new();//mut!!!
   v.push(1);
   ```

3. 访问

   ```rust
   let v = vec![1, 2, 3, 4, 5];
   
   //下标访问,返回引用类型&[]
   let third: &i32 = &v[2];
   println!("第三个元素是 {}", third);
   
   //get()方法访问，返回Option(&T)类型
   match v.get(2) {
       Some(third) => println!("第三个元素是 {third}"),
       None => println!("去你的第三个元素，根本没有！"),
   }
   
   //迭代访问(适用于依次访问的情况)
   let v = vec![1, 2, 3];
   for i in &v {
       println!("{i}");
   }
   ```

   ​	第一种更快，第二种更安全。

   ​	当你确保索引不会越界的时候，就用索引访问，否则用 `.get`。例如，访问第几个数组元素并不取决于我们，而是取决于用户的输入时，用 `.get` 会非常适合，天知道那些可爱的用户会输入一个什么样的数字进来！

4. 同时借用了多个数组元素

   ```rust
   let mut v = vec![1, 2, 3, 4, 5];
   
   let first = &v[0];//不可变借用
   
   v.push(6);//可变借用
   
   println!("The first element is: {first}");//不可变借用在可变借用中被使用
   ```

   ​	这段代码会报错，因为可能会出现特殊情况。就是当地址空间不够用的时候，数组需要动态扩容。从而被整体腾挪，继而使得`first`借用失效。

5. 如何利用动态数组存储不同的类型？

   ​	Vector对于存储的类型要求都相同。面对类型不同的元素，可以通过枚举或者特征对象的方式实现。

   ​	枚举将不同类型的元素包在一个枚举结构里面，特征对象将不同类型的对象用指针指向。

   ```rust
   //枚举实现
   #[derive(Debug)]
   enum IpAddr {
       V4(String),
       V6(String)
   }
   fn main() {
       let v = vec![
           IpAddr::V4("127.0.0.1".to_string()),
           IpAddr::V6("::1".to_string())
       ];
   
       for ip in v {
           show_addr(ip)
       }
   }
   
   fn show_addr(ip: IpAddr) {
       println!("{:?}",ip);
   }
   
   //特征对象实现
   trait IpAddr {
       fn display(&self);
   }
   
   struct V4(String);
   impl IpAddr for V4 {
       fn display(&self) {
           println!("ipv4: {:?}",self.0)
       }
   }
   struct V6(String);
   impl IpAddr for V6 {
       fn display(&self) {
           println!("ipv6: {:?}",self.0)
       }
   }
   
   fn main() {
       let v: Vec<Box<dyn IpAddr>> = vec![
           Box::new(V4("127.0.0.1".to_string())),
           Box::new(V6("::1".to_string())),
       ];
   
       for ip in v {
           ip.display();
       }
   }
   ```

6. Vector常用方法

   ```rust
   //初始化方法
   fn main() {
       let mut v = Vec::with_capacity(10);//创建指定容量的vector
       v.extend([1, 2, 3]);    // 附加数据到 v
       println!("Vector 长度是: {}, 容量是: {}", v.len(), v.capacity());
   
       v.reserve(100);        // 调整 v 的容量，至少要有 100 的容量
       println!("Vector（reserve） 长度是: {}, 容量是: {}", v.len(), v.capacity());
   
       v.shrink_to_fit();     // 释放剩余的容量，一般情况下，不会主动去释放容量
       println!("Vector（shrink_to_fit） 长度是: {}, 容量是: {}", v.len(), v.capacity());
   }
   
   //其他方法
   let mut v =  vec![1, 2];
   assert!(!v.is_empty());         // 检查 v 是否为空
   
   v.insert(2, 3);                 // 在指定索引插入数据，索引值不能大于 v 的长度， v: [1, 2, 3] 
   assert_eq!(v.remove(1), 2);     // 移除指定位置的元素并返回, v: [1, 3]
   assert_eq!(v.pop(), Some(3));   // 删除并返回 v 尾部的元素，v: [1]
   assert_eq!(v.pop(), Some(1));   // v: []
   assert_eq!(v.pop(), None);      // 记得 pop 方法返回的是 Option 枚举值
   v.clear();                      // 清空 v, v: []
   
   let mut v1 = [11, 22].to_vec(); // append 操作会导致 v1 清空数据，增加可变声明
   v.append(&mut v1);              // 将 v1 中的所有元素附加到 v 中, v1: []
   v.truncate(1);                  // 截断到指定长度，多余的元素被删除, v: [11]
   v.retain(|x| *x > 10);          // 保留满足条件的元素，即删除不满足条件的元素
   
   let mut v = vec![11, 22, 33, 44, 55];
   // 删除指定范围的元素，同时获取被删除元素的迭代器, v: [11, 55], m: [22, 33, 44]
   let mut m: Vec<_> = v.drain(1..=3).collect();    
   
   let v2 = m.split_off(1);        // 指定索引处切分成两个 vec, m: [22], v2: [33, 44]
   ```

7. Vector的排序

   ​	在 rust 里，实现了两种排序算法，分别为稳定的排序 `sort` 和 `sort_by`，以及非稳定排序 `sort_unstable` 和 `sort_unstable_by`。

   ```rust
   //对整数进行排序
   fn main() {
       let mut vec = vec![1, 5, 10, 2, 15];    
       vec.sort_unstable();    
       assert_eq!(vec, vec![1, 2, 5, 10, 15]);
   }
   
   //对浮点数进行排序
   //浮点数并不支持比较操作，因为其有NAN这个元素不支持和任何元素比较
   //因此浮点数只是部分可比较，特性为PartialOrd
   fn main() {
       let mut vec = vec![1.0, 5.6, 10.3, 2.0, 15f32];    
       vec.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());    
       assert_eq!(vec, vec![1.0, 2.0, 5.6, 10.3, 15f32]);
   }
   ```

#### 哈希表HashMap

​	所有的K都需要同一类型，所有的V也需要同一类型。

1. 关联函数创建

   ```rust
   use std::collections::HashMap;
   
   // 创建一个HashMap，用于存储宝石种类和对应的数量
   let mut my_gems = HashMap::new();
   
   // 将宝石类型和对应的数量写入表中
   my_gems.insert("红宝石", 1);
   my_gems.insert("蓝宝石", 2);
   my_gems.insert("河边捡的误以为是宝石的破石头", 18);
   ```

2. 迭代器创建

   ​	在实际使用中，不是所有的场景都能 `new` 一个哈希表后，然后悠哉悠哉的依次插入对应的键值对，而是可能会从另外一个数据结构中，获取到对应的数据，最终生成 `HashMap`。

   ​	例如考虑一个场景，有一张表格中记录了足球联赛中各队伍名称和积分的信息，这张表如果被导入到 Rust 项目中，一个合理的数据结构是 `Vec<(String, u32)>` 类型，该数组中的元素是一个个元组，该数据结构跟表格数据非常契合：表格中的数据都是逐行存储，每一个行都存有一个 `(队伍名称, 积分)` 的信息。

   ​	但是在很多时候，又需要通过队伍名称来查询对应的积分，此时动态数组就不适用了，因此可以用 `HashMap` 来保存相关的**队伍名称 -> 积分**映射关系。 理想很丰满，现实很骨感，如何将 `Vec<(String, u32)>` 中的数据快速写入到 `HashMap<String, u32>` 中？

   ​	好在，Rust 为我们提供了一个非常精妙的解决办法：先将 `Vec` 转为迭代器，接着通过 `collect` 方法，将迭代器中的元素收集后，转成 `HashMap`：

   ```rust
   fn main() {
       use std::collections::HashMap;
   
       let teams_list = vec![
           ("中国队".to_string(), 100),
           ("美国队".to_string(), 10),
           ("日本队".to_string(), 50),
       ];
                                               //.into_iter()将列表转换为迭代器
       let teams_map: HashMap<_,_> = teams_list.into_iter().collect();
       			   //此处需要类型标注，因为collect()方法会生成不同的类型
       println!("{:?}",teams_map)
   }
   ```

3. 创建过程中的所有权转移

   ​	这个和其他类型也是一样的。

   - 若类型实现 `Copy` 特征，该类型会被复制进 `HashMap`，因此无所谓所有权
   - 若没实现 `Copy` 特征，所有权将被转移给 `HashMap` 中
   - 若是引用类型，请保证被引用对象的存活时间和`HashMap`一样长

   ```rust
   fn main() {
       use std::collections::HashMap;
   
       let name = String::from("Sunface");
       let age = 18;
   
       let mut handsome_boys = HashMap::new();
       handsome_boys.insert(name, age);
   
       println!("因为过于无耻，{}已经被从帅气男孩名单中除名", name);//报错，所有权已经被转移
       println!("还有，他的真实年龄远远不止{}岁", age);
   }
   
   fn main() {
       use std::collections::HashMap;
   
       let name = String::from("Sunface");
       let age = 18;
   
       let mut handsome_boys = HashMap::new();
       handsome_boys.insert(&name, age);
   
       std::mem::drop(name);
       println!("因为过于无耻，{:?}已经被除名", handsome_boys);//报错，被引用对象已经被释放
       println!("还有，他的真实年龄远远不止{}岁", age);
   }
   ```

4. 查询

   ```rust
   //get()查询
   use std::collections::HashMap;
   
   let mut scores = HashMap::new();
   
   scores.insert(String::from("Blue"), 10);
   scores.insert(String::from("Yellow"), 50);
   
   let team_name = String::from("Blue");
   let score: Option<&i32> = scores.get(&team_name);
   //注意&team_name，如果不使用引用可能会发生所有权的转移
   
   
   
   //循环方式遍历所有的KV对
   use std::collections::HashMap;
   
   let mut scores = HashMap::new();
   
   scores.insert(String::from("Blue"), 10);
   scores.insert(String::from("Yellow"), 50);
   
   for (key, value) in &scores {
       println!("{}: {}", key, value);
   }
   ```

5. 更新

   ```rust
   fn main() {
       use std::collections::HashMap;
   
       let mut scores = HashMap::new();
   
       scores.insert("Blue", 10);
   
       // 覆盖已有的值
       let old = scores.insert("Blue", 20);
       assert_eq!(old, Some(10));
   
       // 查询新插入的值
       let new = scores.get("Blue");
       assert_eq!(new, Some(&20));
   
       // 查询Yellow对应的值，若不存在则插入新值
       let v = scores.entry("Yellow").or_insert(5);
       assert_eq!(*v, 5); // 不存在，插入5
   
       // 查询Yellow对应的值，若不存在则插入新值
       let v = scores.entry("Yellow").or_insert(50);
       assert_eq!(*v, 5); // 已经存在，因此50没有插入
   }
   
   //在已有的值的基础上更新
   use std::collections::HashMap;
   
   let text = "hello world wonderful world";
   
   let mut map = HashMap::new();
   // 根据空格来切分字符串(英文单词都是通过空格切分)
   for word in text.split_whitespace() {
       let count = map.entry(word).or_insert(0);
       //or_insert()方法返回的是&mut v引用,可以用可变引用修改值
       *count += 1;//修改的时候解引用
   }
   
   println!("{:?}", map);
   
   ```



### 认识生命周期

1. 借用检查

   ​	为了保证 Rust 的所有权和借用的正确性，Rust 使用了一个借用检查器(Borrow checker)，来检查我们程序的借用正确性。

   ```rust
   // 'a 引用了 'b的变量 编译器拒绝运行
   {
       let r;                // ---------+-- 'a
                             //          |
       {                     //          |
           let x = 5;        // -+-- 'b  |
           r = &x;           //  |       |
       }                     // -+       |
                             //          |
       println!("r: {}", r); //          |
   }                         // ---------+
   
   
   // 必须确保被引用的变量生命周期比引用变量大才可以
   {
       let x = 5;            // ----------+-- 'b
                             //           |
       let r = &x;           // --+-- 'a  |
                             //   |       |
       println!("r: {}", r); //   |       |
                             // --+       |
   }                         // ----------+
   
   ```

2. 函数中的生命周期

   ​	一般来说，存在多个引用时编译器可能就没办法推导出生命周期了。需要手动标注

   ```rust
   fn main() {
       let string1 = String::from("abcd");
       let string2 = "xyz";
   
       let result = longest(string1.as_str(), string2);
       println!("The longest string is {}", result);
   }
   
   fn longest(x: &str, y: &str) -> &str {
       if x.len() > y.len() {
           x
       } else {
           y
       }
   }
   
   //编译器现在无法知道函数的返回值到底要引用x还是引用y，因为x和y的生命周期可能是不一样的
   //也就是说函数返回值的生命周期可能有两种情况
   //编译器的借用检查也无法推导出返回值的生命周期，因为它不知道 x 和 y 的生命周期跟返回值的生命周期之间的关系是怎样的
   ```

3. 生命周期标注语法

   ​	**生命周期标注并不会改变任何引用的实际作用域**

   ```rust
   &i32        // 一个引用
   &'a i32     // 具有显式生命周期的引用
   &'a mut i32 // 具有显式生命周期的可变引用
   
   //函数签名的生命周期标注
   //这里的标注只能说明first和second和'a活得一样久，具体谁活得更久都是无从而知的
   fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
       if x.len() > y.len() {
           x
       } else {
           y
       }
   }
   ```

   ​	**在通过函数签名指定生命周期参数时，我们并没有改变传入引用或者返回引用的真实生命周期，而是告诉编译器当不满足此约束条件时，就拒绝编译通过**。

   ```rust
   //函数返回值的生命周期应该和其中参数较小的那个一样
   fn main() {
       let string1 = String::from("long string is long");
   
       {
           let string2 = String::from("xyz");
           let result = longest(string1.as_str(), string2.as_str());
           println!("The longest string is {}", result);
       }
       println!("The longest string is {}", result);
       //这行会报错，因为result的生命周期在出花括号的时候就已经结束了
   }
   ```

   ​	作为人类，我们可以很清晰的看出 `result` 实际上引用了 `string1`，因为 `string1` 的长度明显要比 `string2` 长，既然如此，编译器不该如此矫情才对，它应该能认识到 `result` 没有引用 `string2`，让我们这段代码通过。只能说，作为尊贵的人类，编译器的发明者，你高估了这个工具的能力，它真的做不到！而且 Rust 编译器在调教上是非常保守的：当可能出错也可能不出错时，它会选择前者，抛出编译错误。

   ​	生命周期的标注应该取决于函数的功能，例如之前的 `longest` 函数，如果它永远只返回第一个参数 `x`，那你完全可以不为`y`标注生命周期。

   ```rust
   fn longest<'a>(x: &'a str, y: &str) -> &'a str {
       x
   }
   ```

   ​	如果函数的返回值是引用类型，那么这个返回值的生命周期只会来自：

   - 函数参数的生命周期
   - 函数体中某个新建引用的生命周期

   ```rust
   //这段函数会报错 result是函数内部的新变量
   //返回result的引用 可是result随着函数结束就已经被释放
   fn longest<'a>(x: &str, y: &str) -> &'a str {
       let result = String::from("really long string");
       result.as_str()
   }
   
   //这种情况的解决办法是直接把result的所有权交出去 不要返回引用
   fn longest<'a>(_x: &str, _y: &str) -> String {
       String::from("really long string")
   }
   
   fn main() {
      let s = longest("not", "important");
   }
   ```

4. 结构体的生命周期

   ​	我们之前对结构体的使用都停留在非引用类型字段上。之前为什么不在结构体中使用字符串字面量或者字符串切片，而是统一使用 `String` 类型？原因很简单，**后者在结构体初始化时，只要转移所有权即可，而前者，抱歉，它们是引用，它们不能为所欲为。**

   ​	在结构体中使用引用，需要为引用的字段标上生命周期。

   ```rust
   struct ImportantExcerpt<'a> {
       part: &'a str, //生命周期标注
   }
   
   fn main() {
       let novel = String::from("Call me Ishmael. Some years ago...");
       let first_sentence = novel.split('.').next().expect("Could not find a '.'");
       let i = ImportantExcerpt {
           part: first_sentence,
       };
   }
   
   //这段会报错，被引用的字符串生命周期没有结构体的长
   fn main() {
       let i;
       {
           let novel = String::from("Call me Ishmael. Some years ago...");
           let first_sentence = novel.split('.').next().expect("Could not find a '.'");
           i = ImportantExcerpt {
               part: first_sentence,
           };
       }
       println!("{:?}",i);
   }
   ```

5. 生命周期消除

   ```rust
   fn first_word(s: &str) -> &str {
       let bytes = s.as_bytes();
   
       for (i, &item) in bytes.iter().enumerate() {
           if item == b' ' {
               return &s[0..i];
           }
       }
   
       &s[..]
   }
   ```

   对于 `first_word` 函数，它的返回值是一个引用类型，那么该引用只有两种情况：

   - 从参数获取
   - 从函数体内部新创建的变量获取

   如果是后者，就会出现悬垂引用，最终被编译器拒绝，因此只剩一种情况：返回值的引用是获取自参数，这就意味着参数和返回值的生命周期是一样的。**道理很简单，我们能看出来，编译器自然也能看出来，因此，就算我们不标注生命周期，也不会产生歧义。**

   三条生命周期消除规则

   1. **每一个引用参数都会获得独自的生命周期**

      例如一个引用参数的函数就有一个生命周期标注: `fn foo<'a>(x: &'a i32)`，两个引用参数的有两个生命周期标注:`fn foo<'a, 'b>(x: &'a i32, y: &'b i32)`, 依此类推。

   2. **若只有一个输入生命周期(函数参数中只有一个引用类型)，那么该生命周期会被赋给所有的输出生命周期**，也就是所有返回值的生命周期都等于该输入生命周期

      例如函数 `fn foo(x: &i32) -> &i32`，`x` 参数的生命周期会被自动赋给返回值 `&i32`，因此该函数等同于 `fn foo<'a>(x: &'a i32) -> &'a i32`

   3. **若存在多个输入生命周期，且其中一个是 `&self` 或 `&mut self`，则 `&self` 的生命周期被赋给所有的输出生命周期**

      拥有 `&self` 形式的参数，说明该函数是一个 `方法`，该规则让方法的使用便利度大幅提升。

   ```rust
   //例子一
   fn first_word(s: &str) -> &str {} //实际代码
   fn first_word<'a>(s: &'a str) -> &str { // 编译器自动为参数添加生命周期 规则一
   fn first_word<'a>(s: &'a str) -> &'a str { // 编译器自动为返回值添加生命周期 规则二
   
   //例子二
   fn longest(x: &str, y: &str) -> &str { // 实际代码
   fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str //规则一
   //此时会报错，因为编译器无法利用规则二为返回值加上生命周期，因此需要手动标注
   
   ```

6. 静态生命周期

   - 生命周期 `'static` 意味着能和程序活得一样久，例如字符串字面量和特征对象

     ```rust
     let s: &'static str = "我没啥优点，就是活得久，嘿嘿";
     ```

   - 实在遇到解决不了的生命周期标注问题，可以尝试 `T: 'static`，有时候它会给你奇迹

   







### 返回值和错误处理

​	Rust 中的错误主要分为两类：

- **可恢复错误**，通常用于从系统全局角度来看可以接受的错误，例如处理用户的访问、操作等错误，这些错误只会影响某个用户自身的操作进程，而不会对系统的全局稳定性产生影响
- **不可恢复错误**，刚好相反，该错误通常是全局性或者系统性的错误，例如数组越界访问，系统启动时发生了影响启动流程的错误等等，这些错误的影响往往对于系统来说是致命的

#### panic!剖析

1. 被动触发和主动调用

   ```rust
   //被动触发
   fn main() {
       let v = vec![1, 2, 3];
   
       v[99];//数组访问越界 直接panic!()
   }
   
   //主动调用
   fn main() {
       panic!("crash and burn");
   }//你可以在任何你想要触发崩溃的地方调用panic!()
   //当调用执行该宏时，程序会打印出一个错误信息，展开报错点往前的函数调用堆栈，最后退出程序。
   ```

2. backtrace栈展开

   在使用时加上一个环境变量可以获取更详细的栈展开信息（可以得知函数调用链）：

   - Linux/macOS 等 UNIX 系统： `RUST_BACKTRACE=1 cargo run`
   - Windows 系统（PowerShell）： `$env:RUST_BACKTRACE=1 ; cargo run`

   被调用的函数会依次逆序显示出来

3. panic!()的两种终止方式

   默认的方式就是 `栈展开`，这意味着 Rust 会回溯栈上数据和函数调用，因此也意味着更多的善后工作，好处是可以给出充分的报错信息和栈调用信息，便于事后的问题复盘。`直接终止`，顾名思义，不清理数据就直接退出程序，善后工作交与操作系统来负责。

   ```markdown
   [profile.release]
   panic = 'abort' #如果需要直接终止方式，在Cargo.toml文件中设置
   ```

4. 何时使用panic!()

   **你确切的知道你的程序是正确时，可以使用 panic**

   因为 `panic` 的触发方式比错误处理要简单，因此可以让代码更清晰，可读性也更加好，当我们的代码注定是正确时，你可以用 `unwrap` 等方法直接进行处理，反正也不可能 `panic` ：

   ```rust
   use std::net::IpAddr;
   let home: IpAddr = "127.0.0.1".parse().unwrap();
   ```

   **可能导致全局有害状态的时候使用panic**（主要针对那些处理不了的错误）

   有害状态大概分为几类：

   - 非预期的错误
   - 后续代码的运行会受到显著影响
   - 内存安全的问题

   当错误预期会出现时，返回一个错误较为合适，例如解析器接收到格式错误的数据，HTTP 请求接收到错误的参数甚至该请求内的任何错误（不会导致整个程序有问题，只影响该次请求）。**因为错误是可预期的，因此也是可以处理的**。

   当启动时某个流程发生了错误，对后续代码的运行造成了影响，那么就应该使用 `panic`，而不是处理错误后继续运行，当然你可以通过重试的方式来继续。

   

#### 错误处理：返回值Result和？

1. 如何获知变量类型或函数的返回类型

   - 查询官方文档

   - 使用vscode和rust-anlayzer插件跳转去看

   - 定义一个错误的类型让编译器告诉你

     ```rust
     let f: u32 = File::open("hello.txt"); //比如这样，编译器一定会报错，然后给出正确类型
     ```

2. 错误处理枚举

   ```rust
   enum Result<T, E> {
       Ok(T),
       Err(E),
   }
   
   //rustlings习题示例
   //用Result<T,E>的时候要说清楚Ok和Err里面具体包裹的类型
   pub fn generate_nametag_text(name: String) -> Result<String,String> {
       if name.is_empty() {
           // Empty names aren't allowed.
           Err("`name` was empty; it must be nonempty.".into())
       } else {
           Ok("Hi! My name is Beyoncé".into())
       }
   }
   
   #[cfg(test)]
   mod tests {
       use super::*;
   
       #[test]
       fn generates_nametag_text_for_a_nonempty_name() {
           assert_eq!(
               generate_nametag_text("Beyoncé".into()),
               Ok("Hi! My name is Beyoncé".into())
           );
       }
   
       #[test]
       fn explains_why_generating_nametag_text_fails() {
           assert_eq!(
               generate_nametag_text("".into()),
               // Don't change this line
               Err("`name` was empty; it must be nonempty.".into())
           );
       }
   }
   ```

3. match匹配错误和unwrap方法

   ```rust
   //match模式匹配，需要穷尽所有情况
   use std::fs::File;
   use std::io::ErrorKind;
   
   fn main() {
       let f = File::open("hello.txt");
   
       let f = match f {
           Ok(file) => file,
           Err(error) => match error.kind() {
               ErrorKind::NotFound => match File::create("hello.txt") {
                   Ok(fc) => fc,
                   Err(e) => panic!("Problem creating the file: {:?}", e),
               },
               other_error => panic!("Problem opening the file: {:?}", other_error),
           },
       };
   }
   
   //unwrap()和expect()方法
   //如果有值直接取出来，没有值直接崩溃
   use std::fs::File;
   
   fn main() {
       let f = File::open("hello.txt").unwrap();
       let f = File::open("hello.txt").expect("Failed to open hello.txt");
       //expect()和unwrap()差不多，但是可以带上自定义的错误提示信息
   }
   ```

4. 错误传播

   ```rust
   use std::fs::File;
   use std::io::{self, Read};
   
   fn read_username_from_file() -> Result<String, io::Error> {
       // 打开文件，f是`Result<文件句柄,io::Error>`
       let f = File::open("hello.txt");
   
       let mut f = match f {
           // 打开文件成功，将file句柄赋值给f
           Ok(file) => file,
           // 打开文件失败，将错误返回(向上传播)
           Err(e) => return Err(e),
       };
       // 创建动态字符串s
       let mut s = String::new();
       // 从f文件句柄读取数据并写入s中
       match f.read_to_string(&mut s) {
           // 读取成功，返回Ok封装的字符串
           Ok(_) => Ok(s),
           // 将错误向上传播
           Err(e) => Err(e),
       }
   }
   
   //传播错误常见操作符 ？
   use std::fs::File;
   use std::io;
   use std::io::Read;
   
   fn read_username_from_file() -> Result<String, io::Error> {
       let mut f = File::open("hello.txt")?;
       let mut s = String::new();
       f.read_to_string(&mut s)?;
       Ok(s)
   }
   // ？ 是一个宏，可以取代错误处理的部分，并可以自动的进行类型转换，使其在更多位置都可以使用
   
   //？ 实现函数的链式调用
   use std::fs::File;
   use std::io;
   use std::io::Read;
   
   fn read_username_from_file() -> Result<String, io::Error> {
       let mut s = String::new();
   
       File::open("hello.txt")?.read_to_string(&mut s)?;
   
       Ok(s)
   }
   ```

   ```rust
   //rustlings习题
   use std::num::ParseIntError;
   
   pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
       let processing_fee = 1;
       let cost_per_item = 5;
       let qty = item_quantity.parse::<i32>()?;//使用问号操作符代替match
       
       Ok(qty * cost_per_item + processing_fee)
       
   }
   
   #[cfg(test)]
   mod tests {
       use super::*;
   
       #[test]
       fn item_quantity_is_a_valid_number() {
           assert_eq!(total_cost("34"), Ok(171));
       }
   
       #[test]
       fn item_quantity_is_an_invalid_number() {
           assert_eq!(
               total_cost("beep boop").unwrap_err().to_string(),
               "invalid digit found in string"
           );
       }
   }
   
   ```

5. ？ 操作符注意的点

   ```rust
   fn first(arr: &[i32]) -> Option<&i32> {
      arr.get(0)? //这里会报错，？ 操作符需要一个变量来承载正确的值
   }
   //这个函数只会返回 Some(&i32) 或者 None，只有错误值能直接返回，正确的值不行，所以如果数组中存在 0 号元素，那么函数第二行使用 ? 后的返回类型为 &i32 而不是 Some(&i32)。
   ```

   ？ 只可以用于以下的形式

   - `let v = xxx()?;`
   - `xxx()?.yyy()?;`



### 包和模块

#### 包

1. 包（Crate）

   一个独立的可编译单元，它编译后会生成一个可执行文件或者一个库。

   **一个包会将相关联的功能打包在一起，使得该功能可以很方便的在多个项目中分享。**例如标准库中没有提供但是在三方库中提供的 `rand` 包，它提供了随机数生成的功能，我们只需要将该包通过 `use rand;` 引入到当前项目的作用域中，就可以在项目中使用 `rand` 的功能：`rand::XXX`。

   **同一个包中不能有同名的类型，但是在不同包中就可以。**例如，虽然 `rand` 包中，有一个 `Rng` 特征，可是我们依然可以在自己的项目中定义一个 `Rng`，前者通过 `rand::Rng` 访问，后者通过 `Rng` 访问，对于编译器而言，这两者的边界非常清晰，不会存在引用歧义。

2. 项目（Package）

   一个 `Package` 只能包含**一个**库(library)类型的包，但是可以包含**多个**二进制可执行类型的包。

   ```markdown
   $ cargo new my-project
        Created binary (application) `my-project` package
   $ ls my-project
   Cargo.toml
   src
   $ ls my-project/src
   main.rs
   ```

   使用 `cargo run` 可以运行该项目，输出：`Hello, world!`

3. 库（library）

   ```markdown
   $ cargo new my-lib --lib
        Created library `my-lib` package
   $ ls my-lib
   Cargo.toml
   src
   $ ls my-lib/src
   lib.rs
   ```

   原因是库类型的 `Package` 只能作为三方库被其它项目引用，而不能独立运行，只有之前的二进制 `Package` 才可以运行。

4. Package结构示例

   ```markdown
   .
   ├── Cargo.toml
   ├── Cargo.lock
   ├── src
   │   ├── main.rs //默认二进制包，编译后生成和Package同名文件
   │   ├── lib.rs   //唯一库包Liarbry
   │   └── bin        //其他二进制包
   │       └── main1.rs
   │       └── main2.rs
   ├── tests    //集成测试文件
   │   └── some_integration_tests.rs
   ├── benches   //基准测试文件
   │   └── simple_bench.rs
   └── examples   //项目示例
       └── simple_example.rs
   ```

#### 模块Module

1. 嵌套模块

   ​	**所有模块均被定义在同一个文件中！！！**

   ```rust
   // 餐厅前厅，用于吃饭
   mod front_of_house { //mod关键字创建模块，后面紧跟着模块名称
       //模块内部可以定义各种Rust类型
       mod hosting {//模块可以嵌套
           fn add_to_waitlist() {}
   
           fn seat_at_table() {}
       }
   
       mod serving {
           fn take_order() {}
   
           fn serve_order() {}
   
           fn take_payment() {}
       }
   }
   ```

2. 用路径引用模块

   想要调用一个函数，就需要知道它的路径，在 Rust 中，这种路径有两种形式：

   - **绝对路径**，从包根开始，路径名以包名或者 `crate` 作为开头
   - **相对路径**，从当前模块开始，以 `self`，`super` 或当前模块的标识符作为开头

   ```rust
   mod front_of_house {
       mod hosting {
           fn add_to_waitlist() {}
       }
   }
   
   pub fn eat_at_restaurant() {
       // 绝对路径
       crate::front_of_house::hosting::add_to_waitlist();
   
       // 相对路径
       front_of_house::hosting::add_to_waitlist();
   }
   ```

3. super引用模块

   ​	`super` 代表的是父模块为开始的引用方式，非常类似于文件系统中的 `..` 语法

   ```rust
   fn serve_order() {}
   
   // 厨房模块
   mod back_of_house {
       fn fix_incorrect_order() {
           cook_order();
           super::serve_order();
       }
   
       fn cook_order() {}
   }
   ```

4. self关键字引用模块

   ```rust
   fn serve_order() {
       self::back_of_house::cook_order()//其实就是引用自身模块的项
   }
   
   mod back_of_house {
       fn fix_incorrect_order() {
           cook_order();
           crate::serve_order();
       }
   
       pub fn cook_order() {}
   }
   ```

5. 可见性

   ​	Rust 出于安全的考虑，默认情况下，所有的类型都是私有化的，包括函数、方法、结构体、枚举、常量，是的，就连模块本身也是私有化的。

   - pub关键字设置可见性

     ```rust
     mod front_of_house {
         pub mod hosting { //hosting模块可见
             fn add_to_waitlist() {}
         }//add_to_waitlist()仍然不可见，如果需要访问，还得把该函数也标记为pub
     }
     ```

     ​	在实际项目中，一个模块需要对外暴露的数据和 API 往往就寥寥数个，如果将模块标记为可见代表着内部项也全部对外可见，那你是不是还得把那些不可见的，一个一个标记为 `private`？反而是更麻烦的多。

   - 结构体和枚举的可见性

     - 将结构体设置为 `pub`，但它的所有字段**依然是私有**的
     - 将枚举设置为 `pub`，它的所有字段**也将对外可见**

     原因在于，枚举和结构体的使用方式不一样。如果枚举的成员对外不可见，那该枚举将一点用都没有，因此枚举成员的可见性自动跟枚举可见性保持一致，这样可以简化用户的使用。

     而结构体的应用场景比较复杂，其中的字段也往往部分在 A 处被使用，部分在 B 处被使用，因此无法确定成员的可见性，那索性就设置为全部不可见，将选择权交给程序员。



#### use引入模块以及受限可见性

1. 避免同名的引用

   ```rust
   use std::fmt;
   use std::io;
   
   //模块::名称
   fn function1() -> fmt::Result {
       // --snip--
   }
   
   fn function2() -> io::Result<()> {
       // --snip--
   }
   
   //as别名引用
   use std::fmt::Result;
   use std::io::Result as IoResult;//赋予一个新的名称
   
   fn function1() -> Result {
       // --snip--
   }
   
   fn function2() -> IoResult<()> {
       // --snip--
   }
   ```

2. 引入项再导出

   ```rust
   mod front_of_house {
       pub mod hosting {
           pub fn add_to_waitlist() {}
       }
   }
   
   //pub use关键字
   //外部项引入到当前作用域时会被自动设置为私有，如果你想让外界能够访问它
   //那么你可以使用pub use关键字使其对外可见
   pub use crate::front_of_house::hosting;
   
   pub fn eat_at_restaurant() {
       hosting::add_to_waitlist();
       hosting::add_to_waitlist();
       hosting::add_to_waitlist();
   }
   ```

3. 简化引入

   ```rust
   //这种一行一行的引入属实是太过于麻烦，要用很多次use关键字
   use std::collections::HashMap;
   use std::collections::BTreeMap;
   use std::collections::HashSet;
   
   use std::cmp::Ordering;
   use std::io;
   
   //使用{}一起引入进来
   use std::collections::{HashMap,BTreeMap,HashSet};
   use std::{cmp::Ordering, io};
   
   //如果你需要同时引入某个模块和模块中的项
   //比如这样子
   use std::io;
   use std::io::Write;
   
   //可以用self来简化
   use std::io::{self, Write};
   ```

   ```rust
   //通过*引入整个模块的所有项
   //需要注意，因为你可能不清楚具体引入了什么
   use std::collections::*;
   
   struct HashMap;
   fn main() {
      let mut v =  HashMap::new();
       //由于本地有HashMap的定义，因此引入的模块HashMap就不会生效
       //本地同名类型的优先级更高
      v.insert("a", 1);
   }
   ```

4. 限制可见性语法

   虽然对外可见，但不是所有人都可见。

   `pub(crate)` 或 `pub(in crate::a)` 就是限制可见性语法，前者是限制在整个包内可见，后者是通过绝对路径，限制在包内的某个模块内可见，总结一下：

   - `pub` 意味着可见性无任何限制
   - `pub(crate)` 表示在当前包可见
   - `pub(self)` 在当前模块可见
   - `pub(super)` 在父模块可见
   - `pub(in <path>)` 表示在某个路径代表的模块中可见，其中 `path` 必须是父模块或者祖先模块



## 进阶内容

### 闭包和迭代器

#### 闭包

**一种匿名函数，它可以赋值给变量也可以作为参数传递给其它函数，不同于函数的是，它允许捕获调用者作用域中的值**

```rust
fn main() {
   let x = 1;
   let sum = |y| x + y;//作用域中有x，被闭包sum捕获

    assert_eq!(3, sum(2));
}
```

1. 闭包的形式定义

   ```markdown
   |param1, param2,...| {
       语句1;
       语句2;
       返回表达式
   }
   # 闭包中最后一行表达式返回的值，就是闭包的返回值
   ```

2. 使用闭包来简化代码

   ```rust
   use std::thread;
   use std::time::Duration;
   
   //传统函数实现，在workout()函数中调用muuuuu()函数
   //问题在于如果将来不用muuuuu()了，要在许多处位置修改调用的代码
   // 开始健身，好累，我得发出声音：muuuu...
   fn muuuuu(intensity: u32) -> u32 {
       println!("muuuu.....");
       thread::sleep(Duration::from_secs(2));
       intensity
   }
   
   fn workout(intensity: u32, random_number: u32) {
       //将函数名赋值给变量，结合action(intensity)即可调用
       //问题在于如果intensity也变了怎么办
       let action = muuuuu;
       
       //闭包实现
       //将闭包赋值给action，闭包所需要用到的intensity参数由其自己在作用域内捕获
       //这样不用管传参，也不用管函数名(如果未来函数变了，只需要在闭包内部修改其实现即可)
       let action = || {
           println!("muuuu.....");
           thread::sleep(Duration::from_secs(2));
           intensity
       };
   
       if intensity < 25 {
           println!(
               "今天活力满满，先做 {} 个俯卧撑!",
               action()
           );
           println!(
               "旁边有妹子在看，俯卧撑太low，再来 {} 组卧推!",
               action()
           );
       } else if random_number == 3 {
           println!("昨天练过度了，今天还是休息下吧！");
       } else {
           println!(
               "昨天练过度了，今天干干有氧，跑步 {} 分钟!",
               action()
           );
       }
   }
   
   fn main() {
       // 动作次数
       let intensity = 10;
       // 随机值用来决定某个选择
       let random_number = 7;
   
       // 开始健身
       workout(intensity, random_number);
   }
   ```

3. 闭包的类型推导

   ```rust
   let example_closure = |x| x;
   
   let s = example_closure(String::from("hello"));
   let n = example_closure(5);
   //报错，s和n的类型不相同
   //闭包在推导出一种类型之后，后续就会以该类型为准
   ```

4. 捕获作用域中的值

   ```rust
   fn main() {
       let x = 4;
   
       let equal_to_x = |z| z == x;
   
       let y = 4;
   
       assert!(equal_to_x(y));
   }
   ```



#### 迭代器

1. 迭代器和for循环的区别

   ​	**是否通过索引来访问集合**

   ```rust
   //使用索引来访问集合
   let arr = [1, 2, 3];
   for (let i = 0; i < arr.length; i++) {
     console.log(arr[i]);
   }
   
   //使用迭代器访问集合
   let arr = [1, 2, 3];
   for v in arr {
       println!("{}",v);
   }
   ```

   ​	在 Rust 中数组是迭代器吗？因为在之前的代码中直接对数组 `arr` 进行了迭代，答案是 `No`。那既然数组不是迭代器，为啥咱可以对它的元素进行迭代呢？

   ​	简而言之就是**数组实现了 `IntoIterator` 特征，Rust 通过 `for` 语法糖，自动把实现了该特征的数组类型转换为迭代器**（你也可以为自己的集合类型实现此特征），最终让我们可以直接对一个数组进行迭代。

2. 惰性初始化

   ```rust
   let v1 = vec![1, 2, 3];
   
   let v1_iter = v1.iter();
   //创建了迭代器iter(),但是此时不会发生任何迭代行为
   
   for val in v1_iter {
       println!("{}", val);
       //只有在for循环开始后才会开始迭代
       //在此之前不会有任何的性能损耗
   }
   
   ```

3. next()方法

   ```rust
   fn main() {
       let arr = [1, 2, 3];
       let mut arr_iter = arr.into_iter();
   
       assert_eq!(arr_iter.next(), Some(1));
       assert_eq!(arr_iter.next(), Some(2));
       assert_eq!(arr_iter.next(), Some(3));
       assert_eq!(arr_iter.next(), None);
   }
   ```

   果不其然，将 `arr` 转换成迭代器后，通过调用其上的 `next` 方法，我们获取了 `arr` 中的元素，有两点需要注意：

   - `next` 方法返回的是 `Option` 类型，当有值时返回 `Some(i32)`，无值时返回 `None`
   - 遍历是按照迭代器中元素的排列顺序依次进行的，因此我们严格按照数组中元素的顺序取出了 `Some(1)`，`Some(2)`，`Some(3)`
   - 手动迭代必须将迭代器声明为 `mut` 可变，因为调用 `next` 会改变迭代器其中的状态数据（当前遍历的位置等），而 `for` 循环去迭代则无需标注 `mut`，因为它会帮我们自动完成

   总之，`next` 方法对**迭代器的遍历是消耗性的**，每次消耗它一个元素，最终迭代器中将没有任何元素，只能返回 `None`。

4. IntoIterator特征

   ​	因为实现了IntoIterator特征，所以可以通过into_iter()方法变成迭代器，**迭代器自身也实现了IntoIterator特征**。

   - `into_iter` 会夺走所有权
   - `iter` 是借用
   - `iter_mut` 是可变借用

   ```rust
   fn main() {
       let values = vec![1, 2, 3];
   
       for v in values.into_iter() {
           println!("{}", v)
       }
   
       // 下面的代码将报错，因为 values 的所有权在上面 `for` 循环中已经被转移走
       // println!("{:?}",values);
   
       let values = vec![1, 2, 3];
       let _values_iter = values.iter();
   
       // 不会报错，因为 values_iter 只是借用了 values 中的元素
       println!("{:?}", values);
   
       let mut values = vec![1, 2, 3];
       // 对 values 中的元素进行可变借用
       let mut values_iter_mut = values.iter_mut();
   
       // 取出第一个元素，并修改为0
       if let Some(v) = values_iter_mut.next() {
           *v = 0;
       }
   
       // 输出[0, 2, 3]
       println!("{:?}", values);
   }
   ```

   **Iterator 和 IntoIterator 的区别**

   这两个其实还蛮容易搞混的，但我们只需要记住，`Iterator` 就是迭代器特征，只有实现了它才能称为迭代器，才能调用 `next`。

   而 `IntoIterator` 强调的是某一个类型如果实现了该特征，它可以通过 `into_iter`，`iter` 等方法变成一个迭代器。

5. 消费者和适配器

   ​	消费者是迭代器上的方法，它会消费掉迭代器中的元素，然后返回其类型的值，这些消费者都有一个共同的特点：在它们的定义中，都依赖 `next` 方法来消费元素，因此这也是为什么迭代器要实现 `Iterator` 特征，而该特征必须要实现 `next` 方法的原因。

   **消费者适配器**

   如果迭代器的某个方法A在其中调用了`next`方法，那么A就是消费者适配器。

   ```rust
   fn main() {
       let v1 = vec![1, 2, 3];
   
       let v1_iter = v1.iter();
   
       let total: i32 = v1_iter.sum();
   
       assert_eq!(total, 6);
   
       // v1_iter 是借用了 v1，因此 v1 可以照常使用
       println!("{:?}",v1);
   
       // 以下代码会报错，因为 `sum` 拿到了迭代器 `v1_iter` 的所有权
       // println!("{:?}",v1_iter);
   }
   ```

   **迭代器适配器**

   产生一个新的迭代器，**需要一个消费者适配器来收尾**

   ```rust
   let v1: Vec<i32> = vec![1, 2, 3];
   
   v1.iter().map(|x| x + 1); //报错，没有消费者适配器来收尾
   
   let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
   //这里v2需要标注类型，因为collect功能比较强大，可以生成很多种类型
   
   assert_eq!(v2, vec![2, 3, 4]);
   
   //map 会对迭代器中的每一个值进行一系列操作，然后把该值转换成另外一个新值，该操作是通过闭包 |x| x + 1 来完成：最终迭代器中的每个值都增加了 1，从 [1, 2, 3] 变为 [2, 3, 4]。
   ```

   **collect**

   **可以将一个迭代器中的元素收集到指定类型中**

   ```rust
   use std::collections::HashMap;
   fn main() {
       let names = ["sunface", "sunfei"];
       let ages = [18, 18];
       let folks: HashMap<_, _> = names.into_iter().zip(ages.into_iter()).collect();
   
       println!("{:?}",folks);
   }
   ```

   `zip` 是一个迭代器适配器，它的作用就是将两个迭代器的内容压缩到一起，形成 `Iterator<Item=(ValueFromA, ValueFromB)>` 这样的新的迭代器，在此处就是形如 `[(name1, age1), (name2, age2)]` 的迭代器。

   然后再通过 `collect` 将新迭代器中`(K, V)` 形式的值收集成 `HashMap<K, V>`，同样的，这里必须显式声明类型，然后 `HashMap` 内部的 `KV` 类型可以交给编译器去推导，最终编译器会推导出 `HashMap<&str, i32>`，完全正确！

   **闭包作为适配器的参数**

   ```rust
   struct Shoe {
       size: u32,
       style: String,
   }
   
   fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
       shoes.into_iter().filter(|s| s.size == shoe_size).collect()
   }
   ```

   **`filter` 是迭代器适配器，用于对迭代器中的每个值进行过滤。** 它使用闭包作为参数，该闭包的参数 `s` 是来自迭代器中的值，然后使用 `s` 跟外部环境中的 `shoe_size` 进行比较，若相等，则在迭代器中保留 `s` 值，若不相等，则从迭代器中剔除 `s` 值，最终通过 `collect` 收集为 `Vec<Shoe>` 类型。

6. 



### 智能指针

#### Box堆对象分配

1. 将值存储在堆上

   ```rust
   fn main() {
       //将3通过智能指针的方式存在堆上
       //Box实现了deref和drop特征
       let a = Box::new(3);
       println!("a = {}", a);//可以正常打印，println!()隐式的进行deref
   
       //报错，因为表达式不可以隐式的解引用
       //需要显式的解引用*b才可以
       let b = a + 1; // cannot add `{integer}` to `Box<{integer}>`
   }
   //离开作用域，隐式的调用drop()
   ```

2. 避免栈上数据的拷贝

   ```rust
   fn main() {
       // 在栈上创建一个长度为1000的数组
       let arr = [0;1000];
       // 将arr所有权转移arr1，由于 `arr` 分配在栈上，因此这里实际上是直接重新深拷贝了一份数据
       let arr1 = arr;
   
       // arr 和 arr1 都拥有各自的栈上数组，因此不会报错
       println!("{:?}", arr.len());
       println!("{:?}", arr1.len());
   
       // 在堆上创建一个长度为1000的数组，然后使用一个智能指针指向它
       let arr = Box::new([0;1000]);
       // 将堆上数组的所有权转移给 arr1，由于数据在堆上，因此仅仅拷贝了智能指针的结构体，底层数据并没有被拷贝
       // 所有权顺利转移给 arr1，arr 不再拥有所有权
       let arr1 = arr;
       println!("{:?}", arr1.len());
       // 由于 arr 不再拥有底层数组的所有权，因此下面代码将报错
       // println!("{:?}", arr.len());
   }
   ```

3. 处理递归类型

   ​	Rust 需要在编译时知道类型占用多少空间，如果一种类型在编译时无法知道具体的大小，那么被称为动态大小类型 DST。

   ​	其中一种无法在编译时知道大小的类型是**递归类型**：在类型定义中又使用到了自身，或者说该类型的值的一部分可以是相同类型的其它值，这种值的嵌套理论上可以无限进行下去，所以 Rust 不知道递归类型需要多少空间：

   ```rust
   enum List {
       Cons(i32, List),
       Nil,
   }
   ```

   ​	以上就是函数式语言中常见的 `Cons List`，它的每个节点包含一个 `i32` 值，还包含了一个新的 `List`，因此这种嵌套可以无限进行下去，Rust 认为该类型是一个 DST 类型，并给予报错：

   ```console
   error[E0072]: recursive type `List` has infinite size //递归类型 `List` 拥有无限长的大小
    --> src/main.rs:3:1
     |
   3 | enum List {
     | ^^^^^^^^^ recursive type has infinite size
   4 |     Cons(i32, List),
     |               ---- recursive without indirection
   ```

   ​	此时若想解决这个问题，就可以使用我们的 `Box<T>`：

   ```rust
   enum List {
       Cons(i32, Box<List>),
       Nil,
   }
   ```

   ​	只需要将 `List` 存储到堆上，然后使用一个智能指针指向它，即可完成从 DST 到 Sized 类型(固定大小类型)的华丽转变。

4. Box::leak

   ​	它可以消费掉 `Box` 并且强制目标值从内存中泄漏。

   ```rust
   fn main() {
      let s = gen_static_str();
      println!("{}", s);
   }
   
   fn gen_static_str() -> &'static str{
       let mut s = String::new();
       s.push_str("hello, world");
   
       Box::leak(s.into_boxed_str())
   }//返回一个static生命周期的s
   ```

   ​	**如果你需要一个在运行期初始化的值，但是可以全局有效，也就是和整个程序活得一样久**，那么就可以使用 `Box::leak`，例如有一个存储配置的结构体实例，它是在运行期动态插入内容，那么就可以将其转为全局有效，虽然 `Rc/Arc` 也可以实现此功能，但是 `Box::leak` 是性能最高的。




#### Deref解引用

​	何为智能指针？能不让你写出 `****s` 形式的解引用，我认为就是智能: )，**智能指针的名称来源，主要就在于它实现了 `Deref` 和 `Drop` 特征，这两个特征可以智能地帮助我们节省使用上的负担**：

- `Deref` 可以让智能指针像引用那样工作，这样你就可以写出同时支持智能指针和引用的代码，例如 `*T`
- `Drop` 允许你指定智能指针超出作用域后自动执行的代码，例如做一些数据清除等收尾工作

1. 通过*获取引用背后的值（常规的解引用）

   ```rust
   fn main() {
       let x = 5;
       let y = &x;
   
       assert_eq!(5, x);
       assert_eq!(5, *y);//对y解引用，获得值5
   }
   ```

2. 智能指针的解引用

   ​	考虑一下智能指针，它是一个结构体类型，如果你直接对它进行 `*myStruct`，显然编译器不知道该如何办，因此我们可以为智能指针结构体实现 `Deref` 特征。

   ```rust
   //实现deref特征之后的智能指针 可以通过*进行解引用
   fn main() {
       let x = Box::new(1);
       let sum = *x + 1;
   }
   //实现了deref特征的类型 用*解引用时 其实本质进行的是*(y.deref())
   //也就是说先通过deref()方法获得一个常规引用，然后再解引用
   //为什么这样实现?
   //因为所有权系统的存在，如果 deref 方法直接返回一个值，而不是引用，那么该值的所有权将被转移给调用者，而我们不希望调用者仅仅只是 *T 一下，就拿走了智能指针中包含的值。
   
   
   //实现deref特征
   struct MyBox<T>(T);
   
   impl<T> MyBox<T> {
       fn new(x: T) -> MyBox<T> {
           MyBox(x)
       }
   }
   
   use std::ops::Deref;
   
   impl<T> Deref for MyBox<T> {
       type Target = T;//关联类型Target 其实是个泛型
   
       fn deref(&self) -> &Self::Target {
           &self.0 //此处规定如果对Mybox智能指针解引用 那么返回&self.0
       }
   }
   
   
   fn main() {
       let y = MyBox::new(5);
   
       assert_eq!(5, *y);
   }
   
   ```

3. 函数和方法中的隐式Deref转换

   对于函数和方法的传参，Rust 提供了一个极其有用的隐式转换：`Deref `转换。若一个类型实现了 `Deref` 特征，那它的引用在传给函数或方法时，会根据参数签名来决定是否进行隐式的 `Deref` 转换，例如：

   ```rust
   fn main() {
       let s = String::from("hello world");
       display(&s)
   }
   
   fn display(s: &str) {
       println!("{}",s);
   }
   ```

   以上代码有几点值得注意：

   - `String` 实现了 `Deref` 特征，可以在需要时自动被转换为 `&str` 类型
   - `&s` 是一个 `&String` 类型，当它被传给 `display` 函数时，自动通过 `Deref` 转换成了 `&str`
   - 必须使用 `&s` 的方式来触发 `Deref`(**仅引用类型的实参才会触发自动解引用**)

   **连续的隐式Deref转换**

   `Deref` 可以支持连续的隐式转换，直到找到适合的形式为止：

   ```rust
   fn main() {
       let s = MyBox::new(String::from("hello world"));
       display(&s)
   }
   
   fn display(s: &str) {
       println!("{}",s);
   }
   ```

   这里我们使用了之前自定义的智能指针 `MyBox`，并将其通过连续的隐式转换变成 `&str` 类型：**首先 `MyBox` 被 `Deref` 成 `String` 类型，结果并不能满足 `display` 函数参数的要求，编译器发现 `String` 还可以继续 `Deref` 成 `&str`，最终成功的匹配了函数参数。**

   想象一下，假如 `Rust` 没有提供这种隐式转换，我们该如何调用 `display` 函数？

   ```rust
   fn main() {
       let m = MyBox::new(String::from("Rust"));
       display(&(*m)[..]);
   }
   ```

   结果不言而喻，肯定是 `&s` 的方式优秀得多。**总之，当参与其中的类型定义了 `Deref` 特征时，Rust 会分析该类型并且连续使用 `Deref` 直到最终获得一个引用来匹配函数或者方法的参数类型，这种行为完全不会造成任何的性能损耗，因为完全是在编译期完成。**

   但是 `Deref` 并不是没有缺点，缺点就是：如果你不知道某个类型是否实现了 `Deref` 特征，那么在看到某段代码时，并不能在第一时间反应过来该代码发生了隐式的 `Deref` 转换。事实上，不仅仅是 `Deref`，在 Rust 中还有各种 `From/Into` 等等会给阅读代码带来一定负担的特征。还是那句话，一切选择都是权衡，有得必有失，得了代码的简洁性，往往就失去了可读性，Go 语言就是一个刚好相反的例子。

   再来看一下在方法、赋值中自动应用 `Deref` 的例子：

   ```rust
   fn main() {
       let s = MyBox::new(String::from("hello, world"));
       let s1: &str = &s;
       let s2: String = s.to_string();
   }
   ```

   对于 `s1`，我们通过两次 `Deref` 将 `&str` 类型的值赋给了它（**赋值操作需要手动解引用**）；而对于 `s2`，我们在其上直接调用方法 `to_string`，实际上 `MyBox` 根本没有没有实现该方法，能调用 `to_string`，完全是因为编译器对 `MyBox` 应用了 `Deref` 的结果（**方法调用会自动解引用**）。

   

#### Drop特征

1. Drop特征的例子

   ```rust
   struct HasDrop1;
   struct HasDrop2;
   impl Drop for HasDrop1 {
       fn drop(&mut self) {
           println!("Dropping HasDrop1!");
       }
   }
   impl Drop for HasDrop2 {
       fn drop(&mut self) {
           println!("Dropping HasDrop2!");
       }
   }
   struct HasTwoDrops {
       one: HasDrop1,
       two: HasDrop2,
   }
   impl Drop for HasTwoDrops {
       fn drop(&mut self) {
           println!("Dropping HasTwoDrops!");
       }
   }
   
   struct Foo;
   
   impl Drop for Foo {
       fn drop(&mut self) {
           println!("Dropping Foo!")
       }
   }
   
   fn main() {
       let _x = HasTwoDrops {
           two: HasDrop2,
           one: HasDrop1,
       };
       let _foo = Foo;
       println!("Running!");
   }
   ```

   输出如下：

   ```markdown
   Running!
   Dropping Foo!
   Dropping HasTwoDrops!
   Dropping HasDrop1!
   Dropping HasDrop2!
   ```

   有以下几个点需要注意：

   - `Drop` 特征中的 `drop` 方法借用了目标的可变引用，而不是拿走了所有权。因此Rust不允许显式的调用析构函数。如果真的要提前收回所有权，可以通过编译器的`std::mem::drop`来实现（其可以拿走所有权）

     ```rust
     pub fn drop<T>(_x: T)
     ```

   - 结构体中每个字段都有自己的 `Drop`

   - 有关drop的顺序问题

     - **变量级别，按照逆序的方式**
     - **结构体内部，按照顺序的方式**

2. 互斥的Copy和Drop特征

   ```rust
   #[derive(Copy)]
   struct Foo;
   
   impl Drop for Foo {
       fn drop(&mut self) {
           println!("Dropping Foo!")
       }
   }
   ```

   ​	我们无法为一个类型同时实现 `Copy` 和 `Drop` 特征。因为实现了 `Copy` 的特征会被编译器隐式的复制，因此非常难以预测析构函数执行的时间和频率。因此这些实现了 `Copy` 的类型无法拥有析构函数。



#### Rc和Arc实现1vN所有权的机制

​	Rust 所有权机制要求一个值只能有一个所有者，在大多数情况下，都没有问题，但是考虑以下情况：

- 在图数据结构中，多个边可能会拥有同一个节点，该节点直到没有边指向它时，才应该被释放清理
- 在多线程中，多个线程可能会持有同一个数据，但是你受限于 Rust 的安全机制，无法同时获取该数据的可变引用

1. Rc（Reference Count）引用计数

      只可以用于同一个线程的内部。

   ```rust
   fn main() {
       let s = String::from("hello, world");
       // s在这里被转移给a
       let a = Box::new(s);
       // 报错！此处继续尝试将 s 转移给 b
       let b = Box::new(s);
   }
   
   use std::rc::Rc;
   fn main() {
       let a = Rc::new(String::from("hello, world"));//Rc创建新变量的同时计数器会+1
       let b = Rc::clone(&a);
       //Rc::clone()复制一份并且将计数器+1
       //浅拷贝 并不会真的复制数据
   
       assert_eq!(2, Rc::strong_count(&a));
       assert_eq!(Rc::strong_count(&a), Rc::strong_count(&b))
   }
   ```

   可以通过`strong_count()`获得当前计数器值

   ```rust
   use std::rc::Rc;
   fn main() {
           let a = Rc::new(String::from("test ref counting"));
           println!("count after creating a = {}", Rc::strong_count(&a));
           let b =  Rc::clone(&a);
           println!("count after creating b = {}", Rc::strong_count(&a));
           {
               let c =  Rc::clone(&a);
               println!("count after creating c = {}", Rc::strong_count(&c));
           }//c离开作用域 释放 Rc的计数器-1
           println!("count after c goes out of scope = {}", Rc::strong_count(&a));
   }
   ```

   当然，`Rc<T>` 是指向底层数据的不可变的引用，因此你无法通过它来修改数据，这也符合 Rust 的借用规则：要么存在多个不可变借用，要么只能存在一个可变借用。

   ```rust
   use std::rc::Rc;
   
   struct Owner {
       name: String,
       // ...其它字段
   }
   
   struct Gadget {
       id: i32,
       owner: Rc<Owner>,
       // ...其它字段
   }
   
   fn main() {
       // 创建一个基于引用计数的 `Owner`.
       let gadget_owner: Rc<Owner> = Rc::new(Owner {
           name: "Gadget Man".to_string(),
       });
   
       // 创建两个不同的工具，它们属于同一个主人
       let gadget1 = Gadget {
           id: 1,
           owner: Rc::clone(&gadget_owner),
       };
       let gadget2 = Gadget {
           id: 2,
           owner: Rc::clone(&gadget_owner),
       };
   
       // 释放掉第一个 `Rc<Owner>`
       drop(gadget_owner);
   
       // 尽管在上面我们释放了 gadget_owner，但是依然可以在这里使用 owner 的信息
       // 原因是在 drop 之前，存在三个指向 Gadget Man 的智能指针引用，上面仅仅
       // drop 掉其中一个智能指针引用，而不是 drop 掉 owner 数据，外面还有两个
       // 引用指向底层的 owner 数据，引用计数尚未清零
       // 因此 owner 数据依然可以被使用
       println!("Gadget {} owned by {}", gadget1.id, gadget1.owner.name);
       println!("Gadget {} owned by {}", gadget2.id, gadget2.owner.name);
   
       // 在函数最后，`gadget1` 和 `gadget2` 也被释放，最终引用计数归零，随后底层
       // 数据也被清理释放
   }
   ```

   `Rc<T>` 是一个智能指针，实现了 `Deref` 特征，因此你无需先解开 `Rc` 指针，再使用里面的 `T`，而是可以直接使用 `T`，例如上例中的 `gadget1.owner.name`

2. Arc

   ```rust
   use std::rc::Rc;
   use std::thread;
   
   //会报错
   // Rc<T> 不能在线程间安全的传递，实际上是因为它没有实现 Send 特征，而该特征是恰恰是多线程间传递数据的关键
   //由于 Rc<T> 需要管理引用计数，但是该计数器并没有使用任何并发原语，因此无法实现原子化的计数操作，最终会导致计数错误。
   fn main() {
       let s = Rc::new(String::from("多线程漫游者"));
       for _ in 0..10 {
           let s = Rc::clone(&s);
           let handle = thread::spawn(move || {
              println!("{}", s)
           });
       }
   }
   
   //Arc<T>实现了线程之间的Send特征和原子化操作，但是伴随着性能的损耗
   use std::sync::Arc;
   use std::thread;
   
   fn main() {
       let s = Arc::new(String::from("多线程漫游者"));
       for _ in 0..10 {
           let s = Arc::clone(&s);
           let handle = thread::spawn(move || {
              println!("{}", s)
           });
       }
   }
   ```

   

### 多线程并发编程

#### Rust中的并发模型

如果大家学过其它语言的多线程，可能就知道不同语言对于线程的实现可能大相径庭：

- 由于操作系统提供了创建线程的 API，因此部分语言会直接调用该 API 来创建线程，因此最终程序内的线程数和该程序占用的操作系统线程数相等，一般称之为**1:1 线程模型**，例如 Rust。
- 还有些语言在内部实现了自己的线程模型（绿色线程、协程），程序内部的 M 个线程最后会以某种映射方式使用 N 个操作系统线程去运行，因此称之为**M:N 线程模型**，其中 M 和 N 并没有特定的彼此限制关系。一个典型的代表就是 Go 语言。
- 还有些语言使用了 Actor 模型，基于消息传递进行并发，例如 Erlang 语言。

每一种模型都有其优缺点及选择上的权衡，而 Rust 在设计时考虑的权衡就是运行时(Runtime)。

> **运行时是那些会被打包到所有程序可执行文件中的 Rust 代码，根据每个语言的设计权衡，运行时虽然有大有小**（例如 Go 语言由于实现了协程和 GC，运行时相对就会更大一些），但是除了汇编之外，每个语言都拥有它。小运行时的其中一个好处在于最终编译出的可执行文件会相对较小，同时也让该语言更容易被其它语言引入使用。

而绿色线程/协程的实现会显著增大运行时的大小，因此 Rust 只在标准库中提供了 `1:1` 的线程模型，如果你愿意牺牲一些性能来换取更精确的线程控制以及更小的线程上下文切换成本，那么可以选择 Rust 中的 `M:N` 模型，这些模型由三方库提供了实现，例如大名鼎鼎的 `tokio`。

#### 多线程编程

1. 创建线程

   使用`thread::spawn`可以创建线程

   ```rust
   use std::thread;
   use std::time::Duration;
   
   fn main() {
       thread::spawn(|| {
           for i in 1..10 {
               println!("hi number {} from the spawned thread!", i);
               thread::sleep(Duration::from_millis(1));
           }//thread::sleep()会使线程休眠一段时间，这段时间会调用其他的代码来运行
       });
   
       for i in 1..5 {
           println!("hi number {} from the main thread!", i);
           thread::sleep(Duration::from_millis(1));
       }
   }
   ```

   - 线程内部的代码使用闭包来执行
   - **`main` 线程一旦结束，程序就立刻结束，因此需要保持它的存活，直到其它子线程完成自己的任务**

2. 等待子线程的结束

   前面说到`main`线程一旦结束程序就会结束，因此我们需要保持其存活

   通过`handle.join()`可以使当前线程阻塞，直到其子线程结束

   ```rust
   use std::thread;
   use std::time::Duration;
   
   fn main() {
       let handle = thread::spawn(|| {
           for i in 1..5 {
               println!("hi number {} from the spawned thread!", i);
               thread::sleep(Duration::from_millis(1));
           }
       });
   
       handle.join().unwrap();//阻塞，等待子线程结束再往下走
   
       for i in 1..5 {
           println!("hi number {} from the main thread!", i);
           thread::sleep(Duration::from_millis(1));
       }
       handle.join().unwrap();//如果放在这里的话就是两个线程交替执行，都执行完之后结束main()
   }
   ```

3. 在线程闭包中使用move关键字获取变量所有权

   ```rust
   use std::thread;
   
   fn main() {
       let v = vec![1, 2, 3];
   
       let handle = thread::spawn(move || {
       //如果不用move关键字，会报错，因为编译器无法确定在mian()线程的v变量可以存活多久
       //从而其不敢使用
           println!("Here's a vector: {:?}", v);
       });
   
       handle.join().unwrap();
   
       // 下面代码会报错borrow of moved value: `v`
       // println!("{:?}",v);
   }
   ```

4. Rust的线程结束

   ​	在一般的系统编程中，其提供了`kill()`接口将线程杀死，但Rust没有。

   ​	Rust 中线程是如何结束的呢？答案很简单：线程的代码执行完，线程就会自动结束。但是如果线程中的代码不会执行完呢？那么情况可以分为两种进行讨论：

   - 线程的任务是一个循环 IO 读取，任务流程类似：IO 阻塞，等待读取新的数据 -> 读到数据，处理完成 -> 继续阻塞等待 ··· -> 收到 socket 关闭的信号 -> 结束线程，在此过程中，绝大部分时间线程都处于阻塞的状态，因此虽然看上去是循环，CPU 占用其实很小，也是网络服务中最最常见的模型
   - 线程的任务是一个循环，里面没有任何阻塞，包括休眠这种操作也没有，此时 CPU 很不幸的会被跑满，而且你如果没有设置终止条件，该线程将持续跑满一个 CPU 核心，并且不会被终止，直到 `main` 线程的结束

   ```rust
   use std::thread;
   use std::time::Duration;
   fn main() {
       // 创建一个线程A
       let new_thread = thread::spawn(move || {
           // 再创建一个线程B
           thread::spawn(move || {
               loop {
                   println!("I am a new thread.");
               }
           })
       });
   
       // 等待新创建的线程执行完成
       new_thread.join().unwrap();
       println!("Child thread is finish!");
   
       // 睡眠一段时间，看子线程创建的子线程是否还在运行
       thread::sleep(Duration::from_millis(100));
   }
   ```

   从之前的线程结束规则，我们可以猜测程序将这样执行：`A` 线程结束后，由它创建的 `B` 线程仍在疯狂输出，直到 `main` 线程在 100 毫秒后结束。如果你把该时间增加到几十秒，就可以看到你的 CPU 核心 100% 的盛况了-,-

5. 线程屏障(Barrier)

   `Barrier`可以使得多个线程都执行到某个点之后再向后运行。

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
               b.wait();//线程屏障，所有的线程都会在打印完"before wait"之后再向后运行
               println!("after wait");
           }));
       }
   
       for handle in handles {
           handle.join().unwrap();
       }
   }
   ```

6. 线程局部变量

   ```rust
   use std::cell::RefCell;
   use std::thread;
   
   thread_local!(static FOO: RefCell<u32> = RefCell::new(1));
   
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

   



## Rustlings习题整理

map()的用法

```rust
fn vec_map(v: &Vec<i32>) -> Vec<i32> {
    v.iter().map(|element| {
        // TODO: Do the same thing as above - but instead of mutating the
        // Vec, you can just return the new number!
        *element * 2
    }).collect()
}
//map方法由原来的迭代器生成一个新的迭代器，对旧迭代器的每一个方法都调用该闭包
```

字符串和切片操作

```rust
fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    input.trim().to_string()
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    input.to_string() + " world!"
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    input.replace("cars","balloons").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
```

判断字符串和字符串切片的类型

```rust
fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");
    string("red".to_string());
    string(String::from("hi"));
    string("rust is fun!".to_owned()); 
    //to_owned()方法用于从借用的数据中创建一个具有所有权的副本
    //和clone方法的区别是如果传入的参数是引用类型的，可以通过复制获得其所有权
    string_slice("nice weather".into()); 
    string(format!("Interpolation {}", "Station"));
    string_slice(&String::from("abc")[0..1]);
    string_slice("  hello there ".trim());
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    string("mY sHiFt KeY iS sTiCkY".to_lowercase()); 
    //to_lowercase()返回此字符串切片的小写等效项，类型为string
}
```

|      | clone()  | to_owned() |
| ---- | -------- | ---------- |
| T    | T -> T   | T -> T     |
| &T   | &T -> &T | &T -> T    |

模块use

```rust
mod delicious_snacks {
    // TODO: Fix these use statements
    pub use self::fruits::PEAR as fruit;//修改为pub才对外可见
    pub use self::veggies::CUCUMBER as veggie;

    mod fruits {
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie
    );
}
```

比赛统计

```rust
use std::collections::HashMap;

// A structure to store the goal details of a team.
struct Team {
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: String) -> HashMap<String, Team> {
    // The name of the team is the key and its associated struct is the value.
    let mut scores: HashMap<String, Team> = HashMap::new();

    for r in results.lines() {
        let v: Vec<&str> = r.split(',').collect();
        let team_1_name = v[0].to_string();
        let team_1_score: u8 = v[2].parse().unwrap();
        let team_2_name = v[1].to_string();
        let team_2_score: u8 = v[3].parse().unwrap();
        //注意Team是没有实现可加性的，只可以按照Team内部的元素来操作
        let team1 = scores.entry(team_1_name).or_insert(Team{
            goals_scored:0,
            goals_conceded:0
        });
        team1.goals_scored += team_1_score;
        team1.goals_conceded += team_2_score;
        
        let team2 = scores.entry(team_2_name).or_insert(Team{
            goals_scored:0,
            goals_conceded:0
        });
        team2.goals_scored += team_2_score;
        team2.goals_conceded += team_1_score;
    }
    scores
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_results() -> String {
        let results = "".to_string()
            + "England,France,4,2\n"
            + "France,Italy,3,1\n"
            + "Poland,Spain,2,0\n"
            + "Germany,England,2,1\n";
        results
    }

    #[test]
    fn build_scores() {
        let scores = build_scores_table(get_results());

        let mut keys: Vec<&String> = scores.keys().collect();
        keys.sort();
        assert_eq!(
            keys,
            vec!["England", "France", "Germany", "Italy", "Poland", "Spain"]
        );
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(get_results());
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 5);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(get_results());
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 2);
    }
}
```

**quiz2**

首先要观察代码判断其类型，随后用match表达式匹配枚举类型，做出相应的处理

```rust
pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: Complete the function signature!
    pub fn transformer(input: Vec<(String,Command)>) -> Vec<String> {
        // TODO: Complete the output declaration!
        let mut output: Vec<String> = vec![];
        for (string, command) in input.iter() {
            // TODO: Complete the function body. You can do it!
            let applied_string:String = match command{
                Command::Uppercase => string.to_uppercase(),
                Command::Trim => string.trim().to_string(),
                Command::Append(n) => format!("{}{}",string,"bar".repeat(*n)),
            };
            output.push(applied_string);
        }
        output
    }
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    use crate::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}

```

从Option中取出值

```rust
//如果你确定Option中是有值的，可以使用unwrap()方法直接取出来
let my_option: Option<i32> = Some(5); // 一个Option<i32>类型的变量

let value = my_option.unwrap();
println!("The value is: {}", value);

//如果要处理可能有None的情况，可以使用unwrap_or(初始值)
//为None的情况设置一个初始值
let my_option: Option<i32> = Some(5); // 一个Option<i32>类型的变量

let value = my_option.unwrap_or(0); // 如果my_option是None，则使用默认值0
println!("The value is: {}", value);

```

Option的类型问题

```rust
let range = 10;
let mut optional_integers: Vec<Option<i8>> = vec![None];

for i in 1..(range + 1) {
    optional_integers.push(Some(i));
}

let mut cursor = range;

//pop()函数会包着一层Some()在外面
while let Some(Some(integer)) = optional_integers.pop() {
    assert_eq!(integer, cursor);
    cursor -= 1;
}
```

所有权的问题

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        //加ref是为了防止所有权的转移
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
}
```

? 表达式

```rust
impl ParsePosNonzeroError {
    fn from_creation(err: CreationError) -> ParsePosNonzeroError {
        ParsePosNonzeroError::Creation(err)
    }
    // TODO: add another error conversion function here.
    fn from_parseint(err: ParseIntError) -> ParsePosNonzeroError{
        ParsePosNonzeroError::ParseInt(err)
    }
}

fn parse_pos_nonzero(s: &str) -> Result<PositiveNonzeroInteger, ParsePosNonzeroError> {
    // TODO: change this to return an appropriate error instead of panicking
    // when `parse()` returns an error.
    let x: i64 = s.parse().map_err(ParsePosNonzeroError::from_parseint)?;
    PositiveNonzeroInteger::new(x).map_err(ParsePosNonzeroError::from_creation)
}
```

为动态数组Vector实现特征

```rust
trait AppendBar {
    fn append_bar(self) -> Self;
}

// TODO: Implement trait `AppendBar` for a vector of strings.
impl AppendBar for Vec<String>{
    fn append_bar(mut self) -> Self{ //声明可变mut
        self.push("Bar".to_string());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }
}
```

特征约束代替类型

```rust
pub trait Licensed {
    fn licensing_info(&self) -> String {
        "some information".to_string()
    }
}

struct SomeSoftware {}

struct OtherSoftware {}

impl Licensed for SomeSoftware {}
impl Licensed for OtherSoftware {}

// YOU MAY ONLY CHANGE THE NEXT LINE
fn compare_license_types(software:impl Licensed, software_two:impl Licensed) -> bool {
    software.licensing_info() == software_two.licensing_info()
}
//下面有SomeSoftware和OtherSoftware两种类型
//用impl Licensed可以指代他们

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_license_information() {
        let some_software = SomeSoftware {};
        let other_software = OtherSoftware {};

        assert!(compare_license_types(some_software, other_software));
    }

    #[test]
    fn compare_license_information_backwards() {
        let some_software = SomeSoftware {};
        let other_software = OtherSoftware {};

        assert!(compare_license_types(other_software, some_software));
    }
}
```

#[should_panic]

```rust
struct Rectangle {
    width: i32,
    height: i32
}

impl Rectangle {
    // Only change the test functions themselves
    pub fn new(width: i32, height: i32) -> Self {
        if width <= 0 || height <= 0 {
            panic!("Rectangle width and height cannot be negative!")
        }
        Rectangle {width, height}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_width_and_height() {
        // This test should check if the rectangle is the size that we pass into its constructor
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.width, 10); // check width
        assert_eq!(rect.height, 20); // check height
    }

    #[test]
    #[should_panic]
    fn negative_width() {
        // This test should check if program panics when we try to create rectangle with negative width
        let _rect = Rectangle::new(-10, 10);

    }

    #[test]
    #[should_panic]
    fn negative_height() {
        // This test should check if program panics when we try to create rectangle with negative height
        let _rect = Rectangle::new(10, -10);
    }
}
```

迭代器方法

```rust
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().to_string() + c.as_str(),
    }
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    words.iter().map(
        |&word| {
            capitalize_first(word)
        }
    ).collect()
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    words.iter().map(
        |&word| {
            capitalize_first(word)
        }
    ).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
```

宏

```rust
//宏的定义要在使用之前
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}

//使用分号区分不同的模式
#[rustfmt::skip]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    }; //使用分号来区分不同的模式
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    }
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
```



## Rust编程第一课

### 基础知识串讲

1. 指针和引用的区别

   在内存中，一个值被存储到内存中的某个位置，这个位置对应一个内存地址。而**指针是一个持有内存地址的值，可以通过解引用（dereference）来访问它指向的内存地址，理论上可以解引用到任意数据类型**。 

   引用（reference）和指针非常类似，不同的是，**引用的解引用访问是受限的，它只能解引用到它引用数据的类型，不能用作它用。**

   **某些引用除了需要一个指针指向内存地址之外，还需要内存地址的长度和其它信息。**如上一讲提到的指向 “hello world” 字符串的指针，还包含字符串长度和字符串的容量，一共使用了 3 个 word，在 64 位 CPU 下占用 24 个字节，这样比正常指针携带更多信息的指针，我们称之为胖指针（fat pointer）。

   
   
   

### 实用的CLI小工具



