# rcore_lab

## Lab1

```markdown
./os/src
Rust        4 Files   119 Lines
Assembly    1 Files    11 Lines

├── bootloader(内核依赖的运行在 M 特权级的 SBI 实现，本项目中我们使用 RustSBI)
│   └── rustsbi-qemu.bin(可运行在 qemu 虚拟机上的预编译二进制版本)
├── LICENSE
├── os(我们的内核实现放在 os 目录下)
│   ├── Cargo.toml(内核实现的一些配置文件)
│   ├── Makefile
│   └── src(所有内核的源代码放在 os/src 目录下)
│       ├── console.rs(将打印字符的 SBI 接口进一步封装实现更加强大的格式化输出)
│       ├── entry.asm(设置内核执行环境的的一段汇编代码)
│       ├── lang_items.rs(需要我们提供给 Rust 编译器的一些语义项，目前包含内核 panic 时的处理逻辑)
│       ├── linker-qemu.ld(控制内核内存布局的链接脚本以使内核运行在 qemu 虚拟机上)
│       ├── main.rs(内核主函数)
│       └── sbi.rs(调用底层 SBI 实现提供的 SBI 接口)
├── README.md
└── rust-toolchain(控制整个项目的工具链版本)
```

### LibOS的实现过程

#### 写一个能让RUST编译器通过的系统软件

系统软件？ 可以跑在裸机上的程序！

Rust的`std`库依赖于现有OS，而`core`库不依赖OS

当你打印一行Hello，World时离不开OS的层层支持

`#[no_std]`移除Rust标准库

`#[panic_handler]`更多是因为RUST的语言特性，出于安全考虑RUST程序必须有错误处理，实现在标准库中，我们移除`std`之后自己给一个简要的实现即可。

##### OS对应用程序做了什么支持？

在执行应用程序之前，语言标准库和三方库需要为其**初始化**，然后再跳转到入口点开始执行。

初始化工作也实现在`std`标准库中

`#[no_main]`告诉OS没有一般意义的`main`函数。免去初始化的过程……

至此可以编译通过……

#### 生成一个可以加载到Qemu上的OS Bin

为什么要用Linker脚本？

因为链接器默认生成的ELF文件不符合我们想要的格式，qemu加载不了ELF

> ELF文件格式如下：
>
> **ELF 文件头** 位于最前端，它包含了整个文件的基本属性，如文件版本，目标机器型号，程序入口等等。
> **.text 为代码段**，也是反汇编处理的部分，他们是以机器码的形式存储，没有反汇编的过程基本不会有人读懂这些二进制代码的。
> **.data 数据段**，保存的那些已经初始化了的全局静态变量和局部静态变量。
> **.bss 段**， 存放的是未初始化的全局变量和局部静态变量，这个很容易理解，因为在未初始化的情况下，我们单独用一个段来保存，可以不在一开始就分配空间，而是在最终连接成可执行文件的时候，再在.bss 段分配空间。
> **其他段**， 还有一些可选的段，比如.rodata 表示这里存储只读数据， .debug 表示调试信息等等，具体遇到可以查看相关文档。
> **自定义段**，这一块是为了实现用户特殊功能而存在的段，方便扩展，比如我们使用全局变量或者函数之前加上 attribute(section(‘name’)) 就可以吧变量或者函数放到以name 作为段名的段中。
> **段表**，Section Header Table ，是一个重要的部分，它描述了ELF 文件包含的所有段的信息，比如每个段的段名，段长度，在文件中的偏移，读写权限和一些段的其他属性

机器一般只认BIN格式，为什么还要有ELF？

ELF包含的内容比较全面，如果在有OS的情况下，OS是可以解析的。可以单步调试

以下是我们的链接脚本，将OS链接起来，从而可以放置到Qemu上运行。

```assembly
OUTPUT_ARCH(riscv) #目标平台为riscv
ENTRY(_start)  #定义入口地址为_start符号标记的地址
BASE_ADDRESS = 0x80200000;

SECTIONS
{
    . = BASE_ADDRESS; # . 表示当前地址 链接器会从当前地址开始放置采集到的段
    skernel = .; # kernel的起始位置被定义为当前地址

    stext = .;
    .text : {
        *(.text.entry) #为什么先放.text.entry? 因为.text.entry放置了内核第一条指令 
        *(.text .text.*)
    }

    . = ALIGN(4K);
    etext = .;
    srodata = .;
    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    }

    . = ALIGN(4K);
    erodata = .;
    sdata = .;
    .data : {
        *(.data .data.*)
        *(.sdata .sdata.*)
    }

    . = ALIGN(4K);
    edata = .;
    .bss : {
        *(.bss.stack)
        sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
    }

    . = ALIGN(4K);
    ebss = .;
    ekernel = .; #OS程序在此结束 前面链接了OS该有的部分

    /DISCARD/ : {
        *(.eh_frame)
    }
}
```

链接完毕后，虽然我们设置的基地址`BASE_ADDRESS`是`0x80000000`，但是ELF文件的头部会有文件头等内容，qemu没办法加载，因此我们需要将其去除。

#### 为我们的LibOS实现函数调用和初始化

前面的OS可以通过编译，但是功能都阉割的差不多了……

如何恢复这些功能？ 

##### 函数调用

![image-20240417191746525](D:\116\sigs\my_OScamps_blog\一二阶段\rcore_lab.assets\image-20240417191746525-1716128629626-3-1716128810216-10.png)

![image-20240417191722521](D:\116\sigs\my_OScamps_blog\一二阶段\rcore_lab.assets\image-20240417191722521-1716128713411-1-1716128810216-11.png)

​	CPU只有一套寄存器，当遇到多层嵌套调用时怎么办？

​	借助物理内存，将寄存器备份起来——**函数调用上下文**

​	如何借助物理内存？ 来看看栈帧的结构

<img src="D:\116\sigs\my_OScamps_blog\一二阶段\rcore_lab.assets\image-20240417192214371-1716128603881-1-1716128810216-12.png" alt="image-20240417192214371" />

###### 编译器如何理解函数调用

`Prologue`序言的目的是为了保存程序的执行状态（保存返回地址寄存器和堆栈寄存器FP）

`Epilogue`尾声的目的是在执行函数体之后恢复到之前的执行状态（跳转到之前存储的返回地址以及恢复之前保存FP寄存器）

编译器**自动**在函数调用代码的前后加入

```assembly
.global sum_then_double
sum_then_double:
	addi sp, sp, -16		# prologue
	sd ra, 0(sp)			
	
	call sum_to                     # body part 
	li t0, 2
	mul a0, a0, t0
	
	ld ra, 0(sp)			# epilogue
	addi sp, sp, 16
	ret

#上面这段代码是否可以优化成下面这个样子?
#不可以 
#没有存储ra sum_to函数就是一个死循环程序
#归根结底就是因为ra只有一个 面对多层嵌套函数调用就出问题了 所以才需要借助内存的帮助
.global sum_then_double
sum_then_double:		
	
	call sum_to                     # body part 
	li t0, 2
	mul a0, a0, t0
	
	ret
```

##### 初始化

之所以可以调用函数，就是因为OS在函数开始之前完成了初始化

我们需要让LibOS也完成初始化工作

```assembly
    .section .text.entry
    .globl _start
_start:
    la sp, boot_stack_top #分配栈空间
    call rust_main #交出控制权 交出控制权之后我们可以使用RUST代码编程

    .section .bss.stack
    .globl boot_stack_lower_bound
boot_stack_lower_bound:
    .space 4096 * 16  #预留栈空间
    .globl boot_stack_top
boot_stack_top:
```

转交控制权之后我们需要继续完成初始化，也就是**清空BSS段**

```rust
/// clear BSS segment
pub fn clear_bss() {
    extern "C" {
        fn sbss();//sbss()和ebss()在link脚本中已经有定义了
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}
```

#### OS之下还有软件 SBI

```rust
fn sbi_call(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let mut ret;
    unsafe {
        //内嵌汇编      RUST无法表达 但是编译器也不需要去翻译这段汇编
        asm!(
            "li x16, 0",
            "ecall",
            inlateout("x10") arg0 => ret,
            in("x11") arg1,
            in("x12") arg2,
            in("x17") which,
        );
    }
    ret
}
```



### 练习：彩色输出

1. ANSI字符指令分析

   下面给出一条示例指令：

   ```
   echo -e "\x1b[31mhello world\x1b[0m"
   ```

   - `echo`：这是一个常用的命令行工具，用于在标准输出（通常是终端）上显示一行文本。

   - `-e`：这个选项使得`echo`命令**能够解释字符串中的转义序列**。如果不使用`-e`，字符串中的转义字符会被当作普通字符输出。

   - `"\x1b[31mhello world\x1b[0m"`：这个字符串包含了文本（"hello world"）和用于控制文本颜色的ANSI转义码。

     - `\x1b` 是**转义字符（ESC）的十六进制表示**，等价于`\033`和`ESC`键在ASCII码中的表示。
     - `[31m` 是一个ANSI转义序列，用于设置文本颜色。这里的`31`代表红色。
     - `hello world` 是将要在终端中显示的文本。
     - `\x1b[0m` 用于重置颜色到默认状态，确保文本颜色之后的输出不会受到影响。

     <img src="D:\116\sigs\my_OScamps_blog\一二阶段\rcore_lab.assets\image-20240414231141869-1716128810216-13.png" alt="image-20240414231141869" />



## Lab2

### 整体分析

<img src="D:\116\sigs\my_OScamps_blog\一二阶段\rcore_lab.assets\batch-os-detail.png" alt="img" style="zoom:67%;" />

- Qemu将包含应用程序和BatchOS的二进制镜像加载到内存中
- RUSTSBI完成初始化工作(建立内核栈，清空.bss段)之后跳转到BatchOS的起始位置
- BatchOS通过AppManager依次加载各个需要执行的应用到指定的内存位置
- 应用执行过程中可能会通过系统调用请求BatchOS的服务

### 特权级机制

为什么要有特权级机制？

> 本质是因为应用程序和OS杂糅在一起，应用程序不值得被信任。
>
> 因此让应用程序执行在一个受限的，没办法破坏OS的环境中更安全

有了特权级之后需要给应用程序一个接口——系统调用

高特权级软件作为低特权级软件的执行环境——为其提供初始化服务(如RUSTSBI作为bootloader),监管（当应用程序出现异常或特殊情况，需要用到执行环境提供的功能，从而暂停应用程序的运行）



### BatchOS的实现过程

#### 实现可以在用户态运行的应用程序

要点主要是两个

- 调整其内存布局使其可以被正确加载
- 使其可以发出系统调用

调整内存布局是指将其绑定到0x80400000（这是我们约定的程序执行位置），然后在链接的时候将text.entry段放到最前面。

使其发出系统调用关键是系统调用编号和参数的传递

```rust
pub fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe {  
        core::arch::asm!(  // asm!()宏和global_asm!()宏都是用于嵌入汇编代码 asm!()宏可以获取上下文信息(类似闭包) 
            "ecall",
            inlateout("x10") args[0] => ret, //寄存器0同时作为输入和输出 inlateout
            in("x11") args[1],
            in("x12") args[2],
            in("x17") id //寄存器a7约定用于传递系统调用编号
            //有时候不需要把变量绑定到寄存器，此时asm!()宏也可以自动完成变量分配
        );
    }
    ret
}
```

对于特定的系统调用，只需要传递`id`和对应的参数，绑定到对应的寄存器(这个操作需要内嵌汇编，借助编译器上完成)即可。

#### 实现批处理操作系统

脚本`link_app.S`会将应用程序和内核链接为同一个二进制镜像，因此可以利用脚本中的符号来获得各个应用程序的地址，从而实现加载。

使用APPManager来维护批处理OS的所有应用相关的信息

```rust
struct AppManager {
    num_app: usize,//总共的APP数量
    current_app: usize,//当前正在运行中的APP
    app_start: [usize; MAX_APP_NUM + 1],//所有应用程序的起始地址
}
```

load_app解读

```rust
unsafe fn load_app(&self, app_id: usize) {
    if app_id >= self.num_app {
        println!("All applications completed!");
        use crate::board::QEMUExit;
        crate::board::QEMU_EXIT_HANDLE.exit_success();
    }
    println!("[kernel] Loading app_{}", app_id);
    // clear app area
    // 清除的逻辑是从APP_BASE_ADDRESS(指针形式)出发，将一个APP大小的空间清0
    //from_raw_parts_mut() 可变切片
    core::slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, APP_SIZE_LIMIT).fill(0);
    //app_src为应用程序在内核镜像中真正被存放的物理地址
    let app_src = core::slice::from_raw_parts(
        self.app_start[app_id] as *const u8,//应用程序被存放的位置
        self.app_start[app_id + 1] - self.app_start[app_id],//后地址-前地址得到该应用程序大小
    );
    //app_dst为从APP_BASE_ADDRESS出发给该应用程序真正分配的执行空间
    let app_dst = core::slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, app_src.len());
    //BatchOS只能在APP_BASE_ADDRESS位置执行，因此我们复制过来
    app_dst.copy_from_slice(app_src);
    // Memory fence about fetching the instruction memory
    // It is guaranteed that a subsequent instruction fetch must
    // observes all previous writes to the instruction memory.
    // Therefore, fence.i must be executed after we have loaded
    // the code of the next app into the instruction memory.
    // See also: riscv non-priv spec chapter 3, 'Zifencei' extension.
    asm!("fence.i"); //清理cache 保证cache没有上一次留下的东西
}

```



#### RISC-V特权级切换

##### 相关寄存器

以下寄存器会在CPU执行完一条指令准备`trap`时，自动完成相关信息的设置

| CSR 名  | 该 CSR 与 Trap 相关的功能                                    |
| ------- | ------------------------------------------------------------ |
| sstatus | `SPP` 等字段给出 Trap 发生之前 CPU 处在哪个特权级（S/U）等信息 |
| sepc    | 当 Trap 是一个异常的时候，记录 Trap 发生之前执行的最后一条指令的地址 |
| scause  | 描述 Trap 的原因                                             |
| stval   | 给出 Trap 附加信息                                           |
| stvec   | 控制 Trap 处理代码的入口地址                                 |

而在完成`trap`准备返回时，通过`sret`返回，该指令修改`sstatus`字段，然后跳转到`sepc`寄存器指向的位置继续执行

##### 相关栈

用户栈和内核栈

换栈`get_sp` 在内核开始Trap异常处理之前完成

Trap上下文信息会被保存到内核栈中

```rust
pub struct TrapContext {
    pub x: [usize; 32],
    pub sstatus: Sstatus,
    pub sepc: usize,
}
```



##### Trap管理

- __alltraps将Trap上下文保存在内核栈中
- 跳转到trap_handler完成Trap分发和处理(RUST实现，match匹配Trap的类型)
- __restore从内核栈恢复Trap上下文
- sret回到应用程序

以下是_alltraps的实现，_restore是逆过程

```assembly
# os/src/trap/trap.S

.macro SAVE_GP n
    sd x\n, \n*8(sp)
.endm

.align 2
__alltraps:
    csrrw sp, sscratch, sp #交换sscratch和sp 进入内核栈
    # now sp->kernel stack, sscratch->user stack
    # allocate a TrapContext on kernel stack
    addi sp, sp, -34*8 #在内核栈分配栈帧
    # save general-purpose registers 保存通用寄存器 x0和tp(x4)被跳过(不会被修改的)
    sd x1, 1*8(sp)
    # skip sp(x2), we will save it later 
    sd x3, 3*8(sp)
    # skip tp(x4), application does not use it
    # save x5~x31
    .set n, 5
    .rept 27
        SAVE_GP %n
        .set n, n+1
    .endr
    # we can use t0/t1/t2 freely, because they were saved on kernel stack 
    #将sstaus和sepc保存到内核栈中
    #scause和stval可以不存
    csrr t0, sstatus
    csrr t1, sepc
    sd t0, 32*8(sp)
    sd t1, 33*8(sp)
    # read user stack from sscratch and save it on the kernel stack
    # 将用户栈栈顶保存到内核栈中
    csrr t2, sscratch
    sd t2, 2*8(sp)
    # set input argument of trap_handler(cx: &mut TrapContext)
    mv a0, sp
    call trap_handler
```

##### 特殊的Trap

应用程序执行完毕之后的Trap是为了加载下一个应用程序，此时我们需要把新的应用程序加载到0x80400000上。

每一个应用程序跑起来的时候，都需要经历一次从内核态进入用户态的过程。

本质是在内核栈中压入一个**构造好的Trap上下文**，然后__restore即可

```rust
// os/src/trap/context.rs

impl TrapContext {
    pub fn set_sp(&mut self, sp: usize) { self.x[2] = sp; }
    pub fn app_init_context(entry: usize, sp: usize) -> Self {
        let mut sstatus = sstatus::read();
        sstatus.set_spp(SPP::User);
        let mut cx = Self {
            x: [0; 32],
            sstatus,
            sepc: entry,
        };
        cx.set_sp(sp);
        cx
    }
}
```





### 实验：sys_write的安全检查

ch2 中，我们实现了第一个系统调用 `sys_write`，这使得我们可以在用户态输出信息。但是 os 在提供服务的同时，还有保护 os 本身以及其他用户程序不受错误或者恶意程序破坏的功能。

由于还没有实现虚拟内存，我们可以在用户程序中指定一个属于其他程序字符串，并将它输出，这显然是不合理的，因此我们要对 sys_write 做检查：

- sys_write 仅能输出位于程序本身内存空间内的数据，否则报错。





## Lab3

### 本章代码导读

本章的重点是实现对应用之间的协作式和抢占式任务切换的操作系统支持。与上一章的操作系统实现相比，有如下一些不同的情况导致实现上也有差异：

- 多个应用同时放在内存中，所以他们的起始地址是不同的，且地址范围不能重叠
- 应用在整个执行过程中会暂停或被抢占，即会有主动或被动的任务切换

这些实现上差异主要集中在对应用程序执行过程的管理、支持应用程序暂停的系统调用和主动切换应用程序所需的时钟中断机制的管理。

对于第一个不同情况，需要**对应用程序的地址空间布局进行调整，每个应用的地址空间都不相同，且不能重叠。**这并不要修改应用程序本身，而是通过一个脚本 `build.py` 来针对每个应用程序修改链接脚本 `linker.ld` 中的 `BASE_ADDRESS` ，让编译器在编译不同应用时用到的 `BASE_ADDRESS` 都不同，且有足够大的地址间隔。这样就可以让每个应用所在的内存空间是不同的。

对于第二个不同情况，需要**实现任务切换，这就需要在上一章的 Trap 上下文切换的基础上，再加上一个 Task 上下文切换，才能完成完整的任务切换。**这里面的关键数据结构是表示应用执行上下文的 `TaskContext` 数据结构和具体完成上下文切换的汇编语言编写的 `__switch` 函数。一个应用的执行需要被操作系统管理起来，这是通过 `TaskControlBlock` 数据结构来表示应用执行上下文的动态执行过程和状态（运行态、就绪态等）。而为了做好应用程序第一次执行的前期初始化准备， `TaskManager` 数据结构的全局变量实例 `TASK_MANAGER` 描述了应用程序初始化所需的数据， 而对 `TASK_MANAGER` 的初始化赋值过程是实现这个准备的关键步骤。

应用程序可以在用户态执行中**主动暂停**，这需要有新的系统调用 `sys_yield` 的实现来支持；为了支持抢占应用执行的抢占式切换，还要**添加对时钟中断的处理**。有了时钟中断，就可以在确定时间间隔内打断应用的执行，并主动切换到另外一个应用，这部分主要是通过对 `trap_handler` 函数中进行扩展，来完成在时钟中断产生时可能进行的任务切换。 `TaskManager` 数据结构的成员函数 `run_next_task` 来具体实现基于任务控制块的任务切换，并会具体调用 `__switch` 函数完成硬件相关部分的任务上下文切换。

### 多道程序放置和加载——锯齿螈OS

![image-20240426111820382](D:\116\sigs\my_OScamps_blog\一二阶段\rcore_lab.assets\image-20240426111820382-1716128810216-14.png)

这个OS和前一个章节的BatchOS区别不大，最大的区别在于一次将多个程序加载进内存中。

通过脚本build.py为每一个应用程序定制一个脚本，使得每个应用程序有各自不同的地址。

随后各自的linker.ld将应用加载到内存中。

执行时由linker.ld中提供的符号，OS可以知道每个应用的地址，从而在Trap(执行结束或程序异常)的时候，可以切换到对应地址执行下一个应用，不需要像BatchOS那样将新应用copy到0x80200000再执行。

### 任务切换

任务：**一个具有一定独立功能的程序在一个数据集合上的一次动态执行过程**（好像只有THU有这个概念……~~汗~~)

进程还会对很多资源进行管理……任务不会

任务没有彻底的地址空间隔离……任务之间也没有协同……



#### 不同类型的上下文切换

- 函数调用

  ​	当时提到过，为了支持嵌套函数调用，不仅需要硬件平台提供特殊的跳转指令，还需要保存和恢复函数调用上下文 。注意在上述定义中，函数调用包含在**普通控制流**（与异常控制流相对）之内，且**始终用一个固定的栈来保存执行的历史记录**，因此函数调用并**不涉及控制流的特权级切换**。但是我们依然可以将其看成调用者和被调用者两个执行过程的“切换”，**二者的协作体现在它们都遵循调用规范，分别保存一部分通用寄存器**，这样的好处是编译器能够有足够的信息来尽可能减少需要保存的寄存器的数目。虽然当时用了很大的篇幅来说明，但**其实整个过程都是编译器负责完成的，我们只需设置好栈就行了**。

- Trap(异常)控制流

  ​	需要保存和恢复系统调用（Trap）上下文 。当时，为了让内核能够完全掌控应用的执行，且不会被应用破坏整个系统，我们必须利用硬件提供的特权级机制，让应用和内核运行在不同的特权级。应用运行在 U 特权级，它所被允许的操作进一步受限，处处被内核监督管理；而内核运行在 S 特权级，有能力处理应用执行过程中提出的请求或遇到的状况。

- 任务切换

  ​	任务切换是来自**两个不同应用在内核中的 Trap 控制流之间的切换**。当一个应用 Trap 到 S 模式的操作系统内核中进行进一步处理（即进入了操作系统的 Trap 控制流）的时候，其 Trap 控制流可以调用一个特殊的 `__switch` 函数。这个函数表面上就是一个普通的函数调用：在 `__switch` 返回之后，将继续从调用该函数的位置继续向下执行。但是其间却隐藏着复杂的控制流切换过程。具体来说，调用 `__switch` 之后直到它返回前的这段时间，原 Trap 控制流 *A* 会先被暂停并被切换出去， CPU 转而运行另一个应用在内核中的 Trap 控制流 *B* 。然后在某个合适的时机，原 Trap 控制流 *A* 才会从某一条 Trap 控制流 *C* （很有可能不是它之前切换到的 *B* ）切换回来继续执行并最终返回。不过，从实现的角度讲， `__switch` 函数和一个普通的函数之间的核心差别仅仅是它会 **换栈** 。

![../_images/switch.png](D:\116\sigs\my_OScamps_blog\一二阶段\rcore_lab.assets\switch.png)

以下是__switch的具体实现

```assembly
.altmacro
.macro SAVE_SN n
    sd s\n, (\n+2)*8(a0)
.endm
.macro LOAD_SN n
    ld s\n, (\n+2)*8(a1)
.endm
    .section .text
    .globl __switch
__switch:
    # __switch(  该符号在RUST中将被解释为一个函数
    #     current_task_cx_ptr: *mut TaskContext,
    #     next_task_cx_ptr: *const TaskContext
    # ) 从RISC-V调用规范可知 current_task_cx_ptr和next_task_cx_ptr分别会通过寄存器a0/a1传入
    # save kernel stack of current task
    sd sp, 8(a0) #保存栈指针到a0打头偏移地址为8的位置
    # save ra & s0~s11 of current execution
    sd ra, 0(a0)
    .set n, 0  #循环变量n=0 通过.rept 12循环SAVE_SN宏 12次 从而保存a0到a11寄存器
    .rept 12  
        SAVE_SN %n
        .set n, n + 1
    .endr
    # restore ra & s0~s11 of next execution
    ld ra, 0(a1) #加载返回地址
    .set n, 0
    .rept 12
        LOAD_SN %n
        .set n, n + 1
    .endr
    # restore kernel stack of next task
    ld sp, 8(a1) #恢复下一个任务的栈指针 
    ret
```

这里可以看出switch实现和一般的函数调用的区别，一般的函数调用编译器会自动生成代码来保存s0~s11这些通用寄存器，而switch就不会（因为其是汇编语言实现的函数），不会被编译器处理，所以我们需要手动保存这些通用寄存器。

```rust
pub struct TaskContext {
    ra: usize,
    sp: usize,//保存的也是栈指针，但和函数调用的区别在于switch过程中有一次换栈
    s: [usize; 12],
}
```

### 多道程序和协作式调度

![始初龙协作式多道程序操作系统 -- CoopOS总体结构](D:\116\sigs\my_OScamps_blog\一二阶段\rcore_lab.assets\more-task-multiprog-os-detail.png)

任务控制块

```rust
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    UnInit, // 未初始化
    Ready, // 准备运行
    Running, // 正在运行
    Exited, // 已退出
}

#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    pub task_status: TaskStatus,//当前任务执行状态
    pub task_cx: TaskContext,//任务上下文
}

pub struct TaskManager {
    num_app: usize, //总共管理多少个任务
    inner: UPSafeCell<TaskManagerInner>,
}

struct TaskManagerInner {
    tasks: [TaskControlBlock; MAX_APP_NUM],//任务列表
    current_task: usize,//当前执行的任务
}
```

#### 主动放弃yield和退出exit

```rust
//主动放弃
pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

//退出
pub fn sys_exit(exit_code: i32) -> ! {
    println!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

fn mark_current_suspended() {
    TASK_MANAGER.mark_current_suspended();
}

fn mark_current_exited() {
    TASK_MANAGER.mark_current_exited();
}

impl TaskManager {
    fn mark_current_suspended(&self) {
        let mut inner = self.inner.borrow_mut();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Ready;
    }

    fn mark_current_exited(&self) {
        let mut inner = self.inner.borrow_mut();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Exited;
    }
}
```

重点是下面的

```rust
//执行下一个任务
fn run_next_task(&self) {
    if let Some(next) = self.find_next_task() {
        let mut inner = self.inner.exclusive_access();//获得进程控制块的可变借用 从而可以修改
        let current = inner.current_task;
        //println!("task {} start",current);
        inner.tasks[next].task_status = TaskStatus::Running;
        inner.current_task = next;
        let current_task_cx_ptr = &mut inner.tasks[current].task_cx as *mut TaskContext;
        let next_task_cx_ptr = &inner.tasks[next].task_cx as *const TaskContext;
        drop(inner); //前面拿到了inner的可变借用 这里要放弃掉 要不然__switch切换的时候没办法对里面的内容进行修改(一般来说会在函数生命周期结束的时候drop掉)
        // before this, we should drop local variables that must be dropped manually
        unsafe {
            __switch(current_task_cx_ptr, next_task_cx_ptr);
        }
        // go back to user mode 
    } else {
        panic!("All applications completed!");
    }
}

//找到下一个任务
fn find_next_task(&self) -> Option<usize> {
    let inner = self.inner.exclusive_access();
    let current = inner.current_task;
    //这里从current+1开始只是一种调度的方式，你从0开始也一样
    //不过current+1开始会更好一点 从0开始碰到Ready的就执行了 后面可能会饥饿
    (current + 1..current + self.num_app + 1) 
        .map(|id| id % self.num_app)
        .find(|id| inner.tasks[*id].task_status == TaskStatus::Ready)
}
```

#### 第一次进入用户态

和前一个章节的情况是类似的……要把特定任务的任务上下文构造出来，然后压入内核栈顶，然后在switch返回之后restore恢复上下文从而开始执行。

如果是所有程序当中最先执行的那个，需要特殊构造一个unused上下文(后面也不会用到了)来填充switch的参数，主要是为了避免覆盖到其他的数据。

```rust
impl TaskContext {
    pub fn goto_restore(kstack_ptr: usize) -> Self {
        extern "C" { fn __restore(); }
        Self {
            ra: __restore as usize,
            sp: kstack_ptr,
            s: [0; 12],
        }
    }
}

pub fn init_app_cx(app_id: usize) -> usize {
    KERNEL_STACK[app_id].push_context(//上下文压入内核栈顶
        TrapContext::app_init_context(get_base_i(app_id), USER_STACK[app_id].get_sp()),
    )
}
```

### 分时多任务和抢占式调度

![image-20240426140842737](D:\116\sigs\my_OScamps_blog\一二阶段\rcore_lab.assets\image-20240426140842737-1716128810216-15.png)

实现的关键是在trap_handler中新增一个分支，可以检测到S特权级的时钟中断

```rust
const SBI_SET_TIMER: usize = 0;

pub fn set_timer(timer: usize) {//设置mtimecmp的值
    //当计时器mtime大于mtimecmp时，就会触发时钟中断
    sbi_call(SBI_SET_TIMER, timer, 0, 0);//基于sbi_call的计时功能
}

// os/src/timer.rs

use crate::config::CLOCK_FREQ;
const TICKS_PER_SEC: usize = 100;

pub fn set_next_trigger() { //设置好下一次时钟中断的时间点
    set_timer(get_time() + CLOCK_FREQ / TICKS_PER_SEC);
}

match scause.cause() {
    Trap::Interrupt(Interrupt::SupervisorTimer) => {
        set_next_trigger();
        suspend_current_and_run_next();
    }
}
```



### 练习：获取任务信息

#### 需求

ch3 中，我们的系统已经能够支持多个任务分时轮流运行，我们希望引入一个新的系统调用 `sys_task_info` 以获取当前任务的信息，定义如下：

```rust
fn sys_task_info(ti: *mut TaskInfo) -> isize
```

- syscall ID: 410
- 查询当前正在执行的任务信息，任务信息包括任务控制块相关信息（任务状态）、任务使用的系统调用及调用次数、系统调用时刻距离任务第一次被调度时刻的时长（单位ms）。

```rust
struct TaskInfo {
    status: TaskStatus,//任务状态 从TaskControlBlock中可以拿到
    syscall_times: [u32; MAX_SYSCALL_NUM],//任务使用的系统调用及其次数
    time: usize//当前系统调用时刻距离开始时候的时长(就是个计时器)
}
```

#### 简要过程

在TCB中添加相应的记录（事实上也没有比在TCB上添加记录更好的选择，首先是TCB集成了该任务相关的信息，其次是TCB原本就有task_status等记录都内部可变)

```rust
pub struct TaskControlBlock {
  /// The task status in it's lifecycle
  pub task_status: TaskStatus,
  /// The task context
  pub task_cx: TaskContext,
  /// 记录每一个任务的开始时间(第一次被调度进CPU的时间,后续再调度也不会再被修改)
  pub start_time : usize,
  /// 记录每一个系统调用的使用次数(index是系统调用号,value是系统调用的次数)
  pub syscall_times:[u32;MAX_SYSCALL_NUM],
}
```

添加记录之后编译器会报错没有初始化变量 找到对应位置加入相关信息

```rust
let mut tasks = [TaskControlBlock {
    task_cx: TaskContext::zero_init(),
    task_status: TaskStatus::UnInit,
    syscall_times:[0;MAX_SYSCALL_NUM],
    start_time:0, //这里记为0是个标志位 第一次调度的时候发现是0 随后用当前时间替换掉 如果不为0就不再变化(代表当前时间)
}; MAX_APP_NUM];
```

修改完TCB的相关信息之后需要考虑一个点，我们如何给TCB提供这些信息？

##### 时间信息

首先是该任务开始运行的时间……

```rust
fn run_next_task(&self) {
    if let Some(next) = self.find_next_task() {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[next].task_status = TaskStatus::Running;
        if inner.tasks[next].start_time == 0{
            inner.tasks[next].start_time = get_time_us();//当调度下一个任务时，为其提供时间
        }
        ......
}
    
fn run_first_task(&self) -> ! {
    let mut inner = self.inner.exclusive_access();
    let task0 = &mut inner.tasks[0];
    task0.task_status = TaskStatus::Running;
    if task0.start_time == 0{
        task0.start_time = get_time_us();
    }
    ......//所有任务中第一个任务的开始时间也需要被记录
}
```

随后对外提供接口用于获取当前任务的时间和所处于的状态

```rust
impl TaskManager{
    fn get_current_taskcontrolblock_start_time(&self) -> usize{
        let inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].start_time
    }

    fn get_current_taskcontrolblock_status(&self) -> usize{
        let inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].task_status
    }
}

///获得当前任务的起始时间
pub fn get_current_start_time() -> usize{
    TASK_MANAGER.get_current_taskcontrolblock_start_time()
}

///获得当前任务的状态
pub fn get_current_taskcontrolblock_status() -> TaskStatus{
    TASK_MANAGER.get_current_taskcontrolblock_status()
}
```

##### 系统调用次数信息

和时间信息是类似的，需要提供改变其的接口

```rust
impl TaskManager{
	fn add_syscall_times(&self,syscall_id:usize){
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].syscall_times[syscall_id] += 1;
    }

    fn get_syscall_times(&self) -> [u32;500]{
        let inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].syscall_times
    }
}

//发生特定的系统调用 为其增加一次计数
pub fn add_syscall_times(syscall_id:usize){
    TASK_MANAGER.add_syscall_times(syscall_id);
}

//获取特定的系统调用次数
pub fn get_syscall_times() -> [u32;500]{
    TASK_MANAGER.get_syscall_times()
}
```

但是和时间又有所不同，时间信息记录下来之后就不会再有变动 而`syscall`的次数每调用一次都需要增加

```rust
pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    add_syscall_times(syscall_id); //发生系统调用 添加一次次数
    match syscall_id {
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_EXIT => sys_exit(args[0] as i32),
        SYSCALL_YIELD => sys_yield(),
        SYSCALL_GET_TIME => sys_get_time(args[0] as *mut TimeVal, args[1]),
        SYSCALL_TASK_INFO => sys_task_info(args[0] as *mut TaskInfo),
        _ => panic!("Unsupported syscall_id: {}", syscall_id),
    }
}
```

##### TaskInfo实现

```rust
/// YOUR JOB: Finish sys_task_info to pass testcases
//ti变量原本未使用，标记为_ti 此处记得修改
pub fn sys_task_info(ti: *mut TaskInfo) -> isize { 
    trace!("kernel: sys_task_info");
    unsafe{
        *ti = TaskInfo{
            status:get_current_taskcontrolblock_status(),
            syscall_times:get_syscall_times(),
            time: (get_time_us() - get_current_start_time())/1000
        };
    }
    0 //如果没什么问题返回0
}
```

#### 一些记录

报错挺多的，反正就照着编译器一个一个来吧……

类型错误……

<img src="D:\116\sigs\my_OScamps_blog\一二阶段\rcore_lab.assets\image-20240428141340827-1716128810216-16.png" alt="image-20240428141340827" style="zoom:67%;" />

missing documentations for functions

不写注释也不行 ~~笑~~

<img src="D:\116\sigs\my_OScamps_blog\一二阶段\rcore_lab.assets\image-20240428142025031-1716128810216-17.png" alt="image-20240428142025031"  />



## Lab4

### 本章代码导读

![头甲龙操作系统 - Address Space OS总体结构](D:\116\sigs\my_OScamps_blog\一二阶段\rcore_lab.assets\addr-space-os-detail.png)

我们先从简单的地方入手，那当然就是先**改进应用程序**了。具体而言，主要就是把 `linker.ld` 中应用程序的起始地址都改为 `0x10000` ，这是**假定我们操作系统能够通过分页机制把不同应用的相同虚地址映射到不同的物理地址中**。这样我们写应用就不用考虑应用的物理地址布局的问题，能够以一种更加统一的方式编写应用程序，可以忽略掉一些不必要的细节。

为了能够在内核中动态分配内存，我们的第二步需要在内核**增加连续内存分配的功能**，具体实现主要集中在 `os/src/mm/heap_allocator.rs` 中。完成这一步后，我们就可以在内核中用到Rust的堆数据结构了，如 `Vec` 、 `Box` 等，这样内核编程就更加灵活了。

操作系统如果要建立页表（构建虚实地址映射关系），首先要能**管理整个系统的物理内存**，这就需要知道整个计算机系统的物理内存空间的范围，物理内存中哪些区域是空闲可用的，哪些区域放置内核/应用的代码和数据。操作系统内核能够以物理页帧为单位分配和回收物理内存，具体实现主要集中在 `os/src/mm/frame_allocator.rs` 中；也能在虚拟内存中以各种粒度大小来动态分配内存资源，具体实现主要集中在 `os/src/mm/heap_allocator.rs` 中。

页表中的页表项的索引其实是虚拟地址中的虚拟页号，页表项的重要内容是物理地址的物理页帧号。为了能够灵活地在虚拟地址、物理地址、虚拟页号、物理页号之间进行各种转换，在 `os/src/mm/address.rs` 中实现了各种转换函数。

完成上述工作后，基本上就做好了建立页表的前期准备。我们就可以开始建立页表，这主要涉及到页表项的数据结构表示，以及多级页表的起始物理页帧位置和整个所占用的物理页帧的记录。具体实现主要集中在 `os/src/mm/page_table.rs` 中。

一旦使能分页机制，CPU 访问到的地址都是虚拟地址了，那么内核中也将基于虚地址进行虚存访问。所以在给应用添加虚拟地址空间前，**内核自己也会建立一个页表，把整块物理内存通过简单的恒等映射（即虚拟地址映射到对等的物理地址）映射到内核虚拟地址空间中**。后续的应用在执行前，也需要操作系统帮助它建立一个虚拟地址空间。这意味着第三章的初级 `task` 将进化到第四章的拥有独立页表的 `task` 。虚拟地址空间需要有一个数据结构管理起来，这就是 `MemorySet` ，即地址空间这个抽象概念所对应的具象体现。在一个虚拟地址空间中，有代码段，数据段等不同属性且不一定连续的子空间，它们通过一个重要的数据结构 `MapArea` 来表示和管理。围绕 `MemorySet` 等一系列的数据结构和相关操作的实现，主要集中在 `os/src/mm/memory_set.rs` 中。比如内核的页表和虚拟空间的建立在如下代码中：

```
1// os/src/mm/memory_set.rs
2
3lazy_static! {
4  pub static ref KERNEL_SPACE: Arc<Mutex<MemorySet>> = Arc::new(Mutex::new(
5     MemorySet::new_kernel()
6  ));
7}
```

完成到这里，我们就可以使能分页机制了。且我们应该有更加方便的机制来给支持应用运行。在本章之前，都是把应用程序的所有元数据丢弃从而转换成二进制格式来执行，这其实把编译器生成的 ELF 执行文件中大量有用的信息给去掉了，比如代码段、数据段的各种属性，程序的入口地址等。既然有了给应用运行提供虚拟地址空间的能力，我们就可以利用 ELF 执行文件中的各种信息来灵活构建应用运行所需要的虚拟地址空间。在 `os/src/loader.rs` 中可以看到如何获取一个应用的 ELF 执行文件数据，而在 `os/src/mm/memory_set` 中的 `MemorySet::from_elf` 可以看到如何通过解析 ELF 来创建一个应用地址空间。

为此，操作系统需要扩展任务控制块 `TaskControlBlock` 的管理范围，使得操作系统能管理拥有独立页表和单一虚拟地址空间的应用程序的运行。相关主要的改动集中在 `os/src/task/task.rs` 中。

由于代表应用程序运行的任务和管理应用的操作系统各自有独立的页表和虚拟地址空间，所以在操作系统的设计实现上需要考虑两个挑战。第一个挑战是 **页表切换** 。由于系统调用、中断或异常导致的应用程序和操作系统之间的上下文切换不像以前那么简单了，因为在这些处理过程中需要**切换页表**，相关改进可参看 `os/src/trap/trap.S` 。还有就是需要**对来自用户态和内核态的异常/中断分别进行处理**，相关改进可参看 `os/src/trap/mod.rs` 和 [跳板的实现](https://rcore-os.cn/rCore-Tutorial-Book-v3/chapter4/6multitasking-based-on-as.html#term-trampoline) 中的讲解。

第二个挑战是 **查页表以访问不同地址空间的数据** 。在内核地址空间中执行的内核代码常常需要读写应用的地址空间中的数据，这无法简单的通过一次访存来解决，而是需要手动查用户态应用的地址空间的页表，知道用户态应用的虚地址对应的物理地址后，转换成对应的内核态的虚地址，才能访问应用地址空间中的数据。如果访问应用地址空间中的数据跨了多个页，还需要注意处理地址的边界条件。具体可以参考 `os/src/syscall/fs.rs`、 `os/src/mm/page_table.rs` 中的 `translated_byte_buffer` 函数的实现。

实现到这，本章的“头甲龙”操作系统应该就可以给应用程序运行提供一个方便且安全的虚拟地址空间了。

### 在内核中实现动态内存分配

​	为了实现动态内存分配，有几点是比较关键的：初始化堆，分配/释放内存块的函数接口，连续内存分配算法。这几点都是通过使用`alloc crate`来实现的（其实就是调了库）

​	`alloc`需要我们实现一个动态内存分配器，所以主要的工作量就在这里。

```rust
//想要运用alloc crate管理堆空间 就得实现这两个接口
pub unsafe fn alloc(&self, layout: Layout) -> *mut u8;
pub unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout);

//! The global allocator
use crate::config::KERNEL_HEAP_SIZE;
use buddy_system_allocator::LockedHeap;//直接调这个库来帮我们实现

#[global_allocator]
/// heap allocator instance
static HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty();
//LockedHeap已经实现了alloc和dealloc两个必须实现的内存管理接口了

#[alloc_error_handler]
/// panic when heap allocation error occurs
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    panic!("Heap allocation error, layout = {:?}", layout);
}
/// heap space ([u8; KERNEL_HEAP_SIZE])
static mut HEAP_SPACE: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];
//我们希望从内存全局分配器得到的内存是一块被0初始化的字节数组，位于.bss段中
/// initiate heap allocator
pub fn init_heap() {
    unsafe {
        HEAP_ALLOCATOR    
        //LockedHeap是一个被互斥锁保护的类型，在堆操作之前需要获取锁，以防止其他线程同时操作 
            .lock()
            .init(HEAP_SPACE.as_ptr() as usize, KERNEL_HEAP_SIZE);
    }
}
```

### SV39多级页表机制

S特权级下有一个`stap`寄存器是启用分页机制的关键

![../_images/satp.png](D:\116\sigs\my_OScamps_blog\一二阶段\rcore_lab.assets\satp.png)

- `MODE` 控制 CPU 使用哪种页表实现(0表示关闭分页，直接用物理地址；8表示启用SV39机制)

- `ASID` 表示地址空间标识符，这里还没有涉及到进程的概念，我们不需要管这个地方；

- `PPN` 存的是根页表所在的物理页号。这样，给定一个虚拟页号，CPU 就可以从三级页表的根页表开始一步步的将其映射到一个物理页号。

  在多任务系统中，每次切换任务时都必须切换`stap`寄存器，从而完成地址空间的切换。

页表项

![../_images/sv39-pte.png](D:\116\sigs\my_OScamps_blog\一二阶段\rcore_lab.assets\sv39-pte.png)

- V(Valid)：仅当位 V 为 1 时，页表项才是合法的；
- R(Read)/W(Write)/X(eXecute)：分别控制索引到这个页表项的对应虚拟页面是否允许读/写/执行；
- U(User)：控制索引到这个页表项的对应虚拟页面是否在 CPU 处于 U 特权级的情况下是否被允许访问；
- G：暂且不理会；
- A(Accessed)：处理器记录自从页表项上的这一位被清零之后，页表项的对应虚拟页面是否被访问过；
- D(Dirty)：处理器记录自从页表项上的这一位被清零之后，页表项的对应虚拟页面是否被修改过。

页表项的简要实现

```rust
// os/src/mm/page_table.rs

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PageTableEntry {
    pub bits: usize,
}

impl PageTableEntry {
    pub fn new(ppn: PhysPageNum, flags: PTEFlags) -> Self {
        PageTableEntry {
            bits: ppn.0 << 10 | flags.bits as usize,
        }
    }
    pub fn empty() -> Self {
        PageTableEntry {
            bits: 0,
        }
    }
    //取出对应的物理页帧
    pub fn ppn(&self) -> PhysPageNum {
        (self.bits >> 10 & ((1usize << 44) - 1)).into()
    }
    pub fn flags(&self) -> PTEFlags {
        PTEFlags::from_bits(self.bits as u8).unwrap()
    }
}
```

#### 多级页表

![../_images/sv39-full.png](D:\116\sigs\my_OScamps_blog\一二阶段\rcore_lab.assets\sv39-full.png)

每一级页表的内部都是一个线性表，$VPN_0,VPN_1,VPN_2,offset$将轮流作为各级页表的偏移量，从每一级页表所在的$BaseAddress$出发，走对应的“步数”，到达指定的页表项，查询，得到下一级页表的$BaseAddress$ 



### OS对SV39多级页表的管理

首先明确可用的物理内存区段……通过`link_app.S`中提供的符号

- 区间的左端点应该是 `ekernel` 的物理地址以上取整方式转化成的物理页号；
- 区间的右端点应该是 `MEMORY_END` 以下取整方式转化成的物理页号。

#### 分配/回收物理页帧的接口

作为一个能够分配/回收物理页帧的接口起码得有的功能

```rust
//FrameAllocator特征声明 作为一个物理页帧分配器必须实现的三个函数
trait FrameAllocator {
    fn new() -> Self;
    fn alloc(&mut self) -> Option<PhysPageNum>;
    fn dealloc(&mut self, ppn: PhysPageNum);
}
```

随后我们一一实现上面提到的这些功能（此处的栈式分配是一种比较简单的实现

```rust
//我们的分配器声明如下
pub struct StackFrameAllocator {
    current: usize,  //空闲内存的起始物理页号
    end: usize,      //空闲内存的结束物理页号
    recycled: Vec<usize>,
}

impl FrameAllocator for StackFrameAllocator {
    fn new() -> Self {
        Self {
            current: 0,
            end: 0,
            recycled: Vec::new(),
        }
    }
    fn alloc(&mut self) -> Option<PhysPageNum> {
        if let Some(ppn) = self.recycled.pop() {
            Some(ppn.into())
        } else {
            if self.current == self.end {
                None
            } else {
                self.current += 1;
                Some((self.current - 1).into())
            }
        }
    }
    fn dealloc(&mut self, ppn: PhysPageNum) {
        let ppn = ppn.0;
        // validity check
        if ppn >= self.current || self.recycled
            .iter()
            .find(|&v| {*v == ppn})
            .is_some() {
            panic!("Frame ppn={:#x} has not been allocated!", ppn);
        }
        // recycle
        self.recycled.push(ppn);
    }
}

//初始化
impl StackFrameAllocator {
    pub fn init(&mut self, l: PhysPageNum, r: PhysPageNum) {
        self.current = l.0;
        self.end = r.0;
    }
}
```

实现页表之后，需要给外界提供分配和回收的接口

```rust
//将一个新创建的物理页帧和FrameTracker绑定 便于编译器监视其生命周期
pub fn frame_alloc() -> Option<FrameTracker> {
    FRAME_ALLOCATOR
        .exclusive_access()
        .alloc()
        .map(|ppn| FrameTracker::new(ppn))
}

fn frame_dealloc(ppn: PhysPageNum) {
    FRAME_ALLOCATOR
        .exclusive_access()
        .dealloc(ppn);
}

//FrameTracker的实现如下 
//将FrameAllocator分配出来的物理页号用来创建FrameTracker
pub struct FrameTracker {
    pub ppn: PhysPageNum,
}

impl FrameTracker {
    pub fn new(ppn: PhysPageNum) -> Self {
        // page cleaning
        let bytes_array = ppn.get_bytes_array();
        for i in bytes_array {
            *i = 0;
        }
        Self { ppn }
    }
}
```

#### 多级页表管理

首先我们需要给出页表的基本实现

```rust
pub struct PageTable {
    root_ppn: PhysPageNum,
    frames: Vec<FrameTracker>,
}

impl PageTable {
    pub fn new() -> Self {
        let frame = frame_alloc().unwrap();
        PageTable {
            root_ppn: frame.ppn,
            frames: vec![frame],
        }
    }
}
```

其次是拆除和建立虚拟地址映射关系

```rust
// os/src/mm/address.rs

impl VirtPageNum {
    pub fn indexes(&self) -> [usize; 3] {
        let mut vpn = self.0;
        let mut idx = [0usize; 3];
        for i in (0..3).rev() {
            idx[i] = vpn & 511;
            vpn >>= 9;
        }
        idx
    }
}

// os/src/mm/page_table.rs

impl PageTable {
    fn find_pte_create(&mut self, vpn: VirtPageNum) -> Option<&mut PageTableEntry> {
        let idxs = vpn.indexes();
        let mut ppn = self.root_ppn;
        let mut result: Option<&mut PageTableEntry> = None;
        for i in 0..3 {
            let pte = &mut ppn.get_pte_array()[idxs[i]];
            if i == 2 {
                result = Some(pte);
                break;
            }
            if !pte.is_valid() {
                let frame = frame_alloc().unwrap();
                *pte = PageTableEntry::new(frame.ppn, PTEFlags::V);
                self.frames.push(frame);
            }
            ppn = pte.ppn();
        }
        result
    }
    fn find_pte(&self, vpn: VirtPageNum) -> Option<&mut PageTableEntry> {
        let idxs = vpn.indexes();
        let mut ppn = self.root_ppn;
        let mut result: Option<&mut PageTableEntry> = None;
        for i in 0..3 {
            let pte = &mut ppn.get_pte_array()[idxs[i]];
            if i == 2 {
                result = Some(pte);
                break;
            }
            if !pte.is_valid() {
                return None;
            }
            ppn = pte.ppn();
        }
        result
    }
}

//前面提供找到一个特定物理页帧的功能
//此处实现映射
impl PageTable {
    pub fn map(&mut self, vpn: VirtPageNum, ppn: PhysPageNum, flags: PTEFlags) {
        let pte = self.find_pte_create(vpn).unwrap();
        assert!(!pte.is_valid(), "vpn {:?} is mapped before mapping", vpn);
        *pte = PageTableEntry::new(ppn, flags | PTEFlags::V);
    }
    pub fn unmap(&mut self, vpn: VirtPageNum) {
        let pte = self.find_pte(vpn).unwrap();
        assert!(pte.is_valid(), "vpn {:?} is invalid before unmapping", vpn);
        *pte = PageTableEntry::empty();
    }
}
```

### 内核和应用的地址空间

操作系统通过对不同页表的管理，来完成对不同应用和操作系统自身所在的虚拟内存，以及虚拟内存与物理内存映射关系的全面管理。这种管理是建立在 **地址空间** 的抽象上，用来表明正在运行的应用或内核自身所在执行环境中的可访问的内存空间。

逻辑段`MapArea`，**连续可用的虚拟地址**

```rust
pub struct MapArea {
    vpn_range: VPNRange,
    data_frames: BTreeMap<VirtPageNum, FrameTracker>,
    map_type: MapType,
    map_perm: MapPermission,
}

// os/src/mm/memory_set.rs

impl MapArea {
    pub fn new(
        start_va: VirtAddr,
        end_va: VirtAddr,
        map_type: MapType,
        map_perm: MapPermission
    ) -> Self {
        let start_vpn: VirtPageNum = start_va.floor();
        let end_vpn: VirtPageNum = end_va.ceil();
        Self {
            vpn_range: VPNRange::new(start_vpn, end_vpn),
            data_frames: BTreeMap::new(),
            map_type,
            map_perm,
        }
    }
    pub fn map(&mut self, page_table: &mut PageTable) {
        for vpn in self.vpn_range {
            self.map_one(page_table, vpn);
        }
    }
    pub fn unmap(&mut self, page_table: &mut PageTable) {
        for vpn in self.vpn_range {
            self.unmap_one(page_table, vpn);
        }
    }
    /// data: start-aligned but maybe with shorter length
    /// assume that all frames were cleared before
    pub fn copy_data(&mut self, page_table: &mut PageTable, data: &[u8]) {
        assert_eq!(self.map_type, MapType::Framed);
        let mut start: usize = 0;
        let mut current_vpn = self.vpn_range.get_start();
        let len = data.len();
        loop {
            //在start+PAGE_SIZE和len两者之间取最小值 确保copy的数据不会越界
            let src = &data[start..len.min(start + PAGE_SIZE)];
            let dst = &mut page_table
                .translate(current_vpn)
                .unwrap()
                .ppn()
                .get_bytes_array()[..src.len()];
            dst.copy_from_slice(src);
            start += PAGE_SIZE;
            //如果当前的start>=len(意味着已经复制完成了)
            if start >= len {
                break;
            }
            current_vpn.step();
        }
    }
    pub fn map_one(&mut self, page_table: &mut PageTable, vpn: VirtPageNum) {
        let ppn: PhysPageNum;
        //根据映射方式来确定地址
        match self.map_type {
            MapType::Identical => {
                ppn = PhysPageNum(vpn.0);
            }
            MapType::Framed => {
                let frame = frame_alloc().unwrap();
                ppn = frame.ppn;
                self.data_frames.insert(vpn, frame);
            }
        }
        let pte_flags = PTEFlags::from_bits(self.map_perm.bits).unwrap();
        page_table.map(vpn, ppn, pte_flags);
    }
    pub fn unmap_one(&mut self, page_table: &mut PageTable, vpn: VirtPageNum) {
        match self.map_type {
            MapType::Framed => {
                self.data_frames.remove(&vpn);
            }
            _ => {}
        }
        page_table.unmap(vpn);
    }
}
```

地址空间

```rust
pub struct MemorySet {
    page_table: PageTable,
    areas: Vec<MapArea>,
}

// os/src/mm/memory_set.rs

impl MemorySet {
    //新建一个地址空间
    pub fn new_bare() -> Self {
        Self {
            page_table: PageTable::new(),
            areas: Vec::new(),
        }
    }
    fn push(&mut self, mut map_area: MapArea, data: Option<&[u8]>) {
        map_area.map(&mut self.page_table);//建立映射
        if let Some(data) = data {//如果有数据的话写入 没有就算了
            map_area.copy_data(&mut self.page_table, data);
        }
        self.areas.push(map_area);
    }
    /// Assume that no conflicts.
    pub fn insert_framed_area(
        &mut self,
        start_va: VirtAddr, end_va: VirtAddr, permission: MapPermission
    ) {
        self.push(MapArea::new(
            start_va,
            end_va,
            MapType::Framed,
            permission,
        ), None);
    }
    //new_kernel()方法可以创建一段新的内核地址空间
    //基本实现思路是引用linker.ld的外部符号
    //new_bare()方法新建地址空间
    //map_trampoline()映射跳板
    //再调用push()方法将./data ./text ./bss等逻辑段一个一个加入
    pub fn new_kernel() -> Self;
    /// Include sections in elf and trampoline and TrapContext and user stack,
    /// also returns user_sp and entry point.
    pub fn from_elf(elf_data: &[u8]) -> (Self, usize, usize);
}
```

建立用户空间的`from_elf`函数实现如下（有点恶心

```rust
// os/src/mm/memory_set.rs

impl MemorySet {
    /// Include sections in elf and trampoline and TrapContext and user stack,
    /// also returns user_sp and entry point.
    pub fn from_elf(elf_data: &[u8]) -> (Self, usize, usize) {
        let mut memory_set = Self::new_bare();
        // map trampoline 将跳板插入应用地址空间
        //跳板被放在应用地址空间的最高位置(但是其U标志位没开，其实在用户态也访问不了)
        memory_set.map_trampoline();
        // map program headers of elf, with U flag
        let elf = xmas_elf::ElfFile::new(elf_data).unwrap();
        let elf_header = elf.header;
        let magic = elf_header.pt1.magic;
        assert_eq!(magic, [0x7f, 0x45, 0x4c, 0x46], "invalid elf!");//通过魔数来判断其是不是一个合法的ELF文件
        let ph_count = elf_header.pt2.ph_count();//确定program header的数目
        let mut max_end_vpn = VirtPageNum(0);
        for i in 0..ph_count {//遍历所有的program header 将合适的区域加入到地址空间中
            let ph = elf.program_header(i).unwrap();
            if ph.get_type().unwrap() == xmas_elf::program::Type::Load {
                let start_va: VirtAddr = (ph.virtual_addr() as usize).into();
                let end_va: VirtAddr = ((ph.virtual_addr() + ph.mem_size()) as usize).into();
                let mut map_perm = MapPermission::U;
                let ph_flags = ph.flags();
                if ph_flags.is_read() {
                    map_perm |= MapPermission::R;
                }
                if ph_flags.is_write() {
                    map_perm |= MapPermission::W;
                }
                if ph_flags.is_execute() {
                    map_perm |= MapPermission::X;
                }
                let map_area = MapArea::new(start_va, end_va, MapType::Framed, map_perm);
                max_end_vpn = map_area.vpn_range.get_end();
                memory_set.push(
                    map_area,
                    Some(&elf.input[ph.offset() as usize..(ph.offset() + ph.file_size()) as usize]),
                );
            }
        }
        // 用户空间的各个段插入完毕 现在放置一个保护页面和用户栈即可
        // map user stack with U flags
        let max_end_va: VirtAddr = max_end_vpn.into();
        let mut user_stack_bottom: usize = max_end_va.into();
        // guard page
        user_stack_bottom += PAGE_SIZE;
        let user_stack_top = user_stack_bottom + USER_STACK_SIZE;
        memory_set.push(
            MapArea::new(
                user_stack_bottom.into(),
                user_stack_top.into(),
                MapType::Framed,
                MapPermission::R | MapPermission::W | MapPermission::U,
            ),
            None,
        );
        // used in sbrk
        memory_set.push(
            MapArea::new(
                user_stack_top.into(),
                user_stack_top.into(),
                MapType::Framed,
                MapPermission::R | MapPermission::W | MapPermission::U,
            ),
            None,
        );
        // 用户栈和保护页面也放置完毕 现在选出用户空间的次高页面 放置Trap上下文(跳板在最开始就放好了)
        // map TrapContext
        memory_set.push(
            MapArea::new(
                TRAP_CONTEXT_BASE.into(),
                TRAMPOLINE.into(),
                MapType::Framed,
                MapPermission::R | MapPermission::W,
            ),
            None,
        );
        (
            memory_set,
            user_stack_top,
            elf.header.pt2.entry_point() as usize,
        )
    }
}
```



### 练习：重写sys_get_time和sys_task_info

先看看原本的sys_get_time是如何实现的？

```rust
/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}
```

对指针`*ts`所指向的内存空间赋值时间信息，在引入虚存之前应用空间和内核空间之间不存在隔离，二者都可以直接访问到`*ts`所在位置。在引入虚存之后，每个应用以及内核本身都有独立的地址空间，没办法访问了。

因此我们需要想办法，使得OS能够访问到应用所在的位置，需要完成二者地址的翻译。

```rust
/// 虚拟地址到物理地址的转换
// 别忘了把新写的函数在mod.rs中公开出来 要不然用不了
pub fn translated_physical_address(satp: usize,ptr:*const u8) -> usize{
    let page_table = PageTable::from_token(satp);//首先根据token参数找到对应的应用进程的页表
    let va = VirtAddr::from(ptr as usize);//将传入的usize生成对应的虚拟地址(只使用较低的39位)
    //首先根据虚拟地址找到对应的物理页，拿到这个物理页之后调用其ppn()方法得到物理页码
    let ppn = page_table.find_pte(va.floor()).unwrap().ppn();
    //物理页码转换为物理基址 + 偏移量 得到特定地址空间下实际的物理页帧位置
    super::PhysAddr::from(ppn).0 + va.page_offset()
}

/// 为当前任务所在地址空间完成地址翻译
pub fn current_tranlated_physical_address(ptr:*const u8) -> usize{
    let token = TASK_MANAGER.get_current_token();
    translated_physical_address(token, ptr)
} 
```

和mmap那个题目一样，sys_get_time以及sys_task_info也是需要在当前任务下才行

为什么突然写回来了呢……（因为在线CI测了sys_get_time还有sys_task_info 发现自己写的根本就不对

我知道哪里不行了 我的实现没问题 搞得我还重写了一次

每次执行系统调用的时候忘记调用`add_syscall_num`

```rust
/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let _us = get_time_us();
    let ts = current_tranlated_physical_address(_ts as *const u8 ) as *mut TimeVal;
    unsafe {
        *ts = TimeVal{
            sec:_us / 1_000_000,
            usec : _us % 1_000_000,
        }
    }
    0
}



/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info");
    let _ti =  current_tranlated_physical_address(ti as *const u8 ) as *mut TaskInfo;
    unsafe{
        *_ti = TaskInfo{
            status : get_current_taskcontrolblock_status(),
            syscall_times : get_syscall_times(),
            time : (get_time_us() - get_current_start_time()) / 1_000
        }
    }
    0
}
```



#### mmap和munmap

`insert_frame_area`函数是比较值得参考的一个函数

不仅仅是函数实现的功能类似，用法也很值得学习

![image-20240504170912216](D:\116\sigs\my_OScamps_blog\一二阶段\rcore_lab.assets\image-20240504170912216-1716128810216-18.png)

基本实现如下 在`MemorySet`中实现一个映射方法

```rust
impl MemorySet{
	/// 申请地址空间
    pub fn mmap(&mut self,start: usize, len: usize, port: usize) -> isize{
        let  vpnrange = VPNRange::new(VirtAddr::from(start).floor(), VirtAddr::from(start+len).ceil());
        for vpn in vpnrange{
            if let Some(pte) = self.page_table.find_pte(vpn){
                if pte.is_valid(){
                    println!("{}\n",pte.is_valid());
                    println!("The Page you wanted has been alloced to others\n");
                    return -1;//已经被分配过
                }
            }
        }
        println!("No Page has been alloc\n");
        // 缺失物理内存空间不足检查的逻辑

        let mut map_prem = MapPermission::U;
        if (port & 1)!=0{
            println!("R\n");
            map_prem|=MapPermission::R;
        }
        if (port & 2)!=0{
            println!("W\n");
            map_prem|=MapPermission::W;
        }
        if (port & 4)!=0{
            println!("X\n");
            map_prem|=MapPermission::X;
        }
        println!("start_va:{:#x}~end_va:{:#x} map_perm:{:#x}\n",start,start+len,map_prem);
        self.insert_framed_area(VirtAddr::from(start), VirtAddr::from(start + len), map_prem);
        0
    }
    
    /// 去除地址空间
    pub fn munmap(&mut self,start: usize,len: usize)->isize{
        let vpnrange = VPNRange::new(VirtAddr::from(start).floor(), VirtAddr::from(start+len).ceil());
        // 检查未被映射的虚存 地址空间不可以被重复释放
        for vpn in vpnrange{
            let pte = self.page_table.find_pte(vpn);
            if pte.is_none() || !pte.unwrap().is_valid(){
                return -1;
            }
        }
        for vpn in vpnrange{
            for area in &mut self.areas{
                if vpn < area.vpn_range.get_end() && vpn >= area.vpn_range.get_start(){
                    area.unmap_one(&mut self.page_table, vpn);
                }
            }
        }
        0
    }
}
```

简单的参数检查在`sys_mmap`中提供（不需要虚地址的

```rust
// 首先检查start是否按页对齐
if start % PAGE_SIZE != 0{
    return -1;
}
// 检查其余位必须为0的条件
if port & !0x7 != 0{
    return -1;
}
// 检查以下的内存是否具有意义
if !port & 0x7 == 7{
    return -1;
}
// 通过参数检查 调用实现的mmap方法为其分配空间
println!("pass the check\n");
```

随后就是漫长的调试……

这里注意到特殊的一行

`[kernel] PageFault in application, bad addr = {:#x}, bad instruction = {:#x}, kernel killed it.`

先找到这行输出是哪里来的，发现在`Trap_handler`方法里面

然后再考虑，发现其实我的程序连`mmap0`都没正常跑完

但是正确分配了页面，要不然就不会有`start_va:0x10000000~end_va:0x10001000 map_perm:0x16`输出

这里的`0x16`完全没问题（之前看成10进制了）

`0001 0110` 代表`U`,`W`,`R`被置位 而测试用例`mmap0`给的是`3` 也就是`011` 也是对应`W`,`R`

![image-20240504211451098](D:\116\sigs\my_OScamps_blog\一二阶段\rcore_lab.assets\image-20240504211451098-1716128810216-19.png)

妈的 我知道怎么搞了 之所以会不断出现`The Page you wanted has been alloced to others`的报错信息是因为之前在`sys_mmap`方法中对MemorySet中`mmap()`的调用是这样子的

```rust
// 获取内核实例 取得所有权完成分配(这样不对 你并没有找到实际你要分配的位置) 实际上你是给内核多次分配了 所以才这样子报错
let num = KERNEL_SPACE.exclusive_access().mmap(start, len, port);
//之所以这样子写主要还是因为参考了前面insert_frame_area的用法 没有具体考虑他使用的上下文
```

事实上应该找到当前运行的任务，只有当前运行的任务是知道自己的地址空间信息的，具体在`TCB`里面有一项`memory_set` 

这里也走了点弯路 一开始我的想法是在`TaskControlBlock`中实现一个`get_current_tasks_area`（类似于之前`get_tasks_start_time`一样拿到时间）拿到`memory_set`的所有权或者引用之后，在`sys_mmap()`里面再用得到的`memory_set`来调用（我个人感觉主要还是仿照了前面代码的思路，就非要拿到一个类似于`KERNEL_SPACE`的地址空间，事实上没必要）

实际上你在`TaskManager`里面直接实现映射的功能就可以了，然后对外提供接口，具体如下

```rust
impl TaskManager{
	/// 为当前地址空间完成内存映射
    pub fn mmap_current_task(&self, start: usize, len: usize, port: usize) -> isize {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        let memory_set = &mut inner.tasks[current].memory_set;
        memory_set.mmap(start, len, port)
    }

    /// 为当前地址空间解映射
    pub fn munmap_current_task(&self,start: usize,len: usize) -> isize{
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        let memory_set = &mut inner.tasks[current].memory_set;
        memory_set.munmap(start, len)
    }
}

/// 为当前地址空间完成地址映射
pub fn mmap_current_task(start: usize,len: usize,port: usize) -> isize{
    TASK_MANAGER.mmap_current_task(start, len, port)
}
```

修改`sys_mmap()`的实现 使其直接调用`mmap_current_task`就可以了

```rust
pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
    trace!("kernel: sys_mmap");
    // 首先检查start是否按页对齐
    if start % PAGE_SIZE != 0{
        return -1;
    }
    // 检查其余位必须为0的条件
    if port & !0x7 != 0{
        return -1;
    }
    // 检查以下的内存是否具有意义
    if !port & 0x7 == 7{
        return -1;
    }
    // 通过参数检查 调用实现的mmap方法为其分配空间
    println!("pass the check\n");
    // 获取内核实例 取得所有权完成分配(这样不对 你并没有找到实际你要分配的位置) 实际上你是给内核多次分配了 所以才这样子报错
    //let num = KERNEL_SPACE.exclusive_access().mmap(start, len, port);

    //问题在于如何获得当前应用空间所在的MemorySet
    //妈的 找到了 在TCB里面有 byd受不了了
    //最好的方式其实还是在TCB里面把地址空间映射了 然后再封装出来
    let num = mmap_current_task(start, len, port);
    //let map_area = MapArea::new(start_va, end_va, MapType::Framed, map_perm);
    //let num = map_area.exclusive_access().mmap(start, len, port);
    if num == 0{
        0
    }else{
        -1
    }
}
```

这里可以看到`mmap`的测试全部通过……里面有一堆调试的输出……

<img src="D:\116\sigs\my_OScamps_blog\一二阶段\rcore_lab.assets\image-20240505003103819-1716128810217-21.png" alt="image-20240505003103819" style="zoom:67%;" />

实现了`mmap`之后`munmap`就比较简单了

这里写一个点 关于`munmap`的最后一个测试用例

```rust
fn main() -> i32 {
    let start: usize = 0x10000000;
    let len: usize = 4096;
    let prot: usize = 3;
    assert_eq!(0, mmap(start, len, prot));
    assert_eq!(munmap(start, len + 1), -1);
    //这个测试用例的意图应该是不允许释放部分空间 但确实不太严格
    assert_eq!(munmap(start + 1, len - 1), -1);
    println!("Test 04_6 ummap2 OK!");
    0
}
```

下面给一个比较滑头的办法 检查一下是不是页对齐就行（start硬编码写死了 所以其实你怎么写都差不多

```rust
// YOUR JOB: Implement munmap.
pub fn sys_munmap(start: usize, len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    if start % PAGE_SIZE != 0{
        return -1;
    }
    let num = munmap_current_task(start, len);
    num
}
```

按理说应该是实现一个检测解除映射范围和现有的映射区域是否完全一致的方法

明天再想吧……先看看能不能过在线CI



#### 一些其他

懂了 在线CI看不到报错 我就说为什么lab4的测例全过了assert断言还是不行

![image-20240505014117148](D:\116\sigs\my_OScamps_blog\一二阶段\rcore_lab.assets\image-20240505014117148-1716128810216-20.png)

这里可以看到比较详细的信息

这里记录一下回退的点 本地执行CI之后需要删除一些未跟踪的文件

```markdown
git clean -f  删除未跟踪的文件（不包括目录）
git clean -fd 删除未跟踪的文件和目录
```



![image-20240505171002449](D:\116\sigs\my_OScamps_blog\一二阶段\rcore_lab.assets\image-20240505171002449-1716128810217-22.png)

解决了 太sb了



## Lab5

### 之前的测例实现过程

经典内容……

注意一个点 在Lab5里面把TaskManager拆分成了TaskManager和processor两个数据结构

不过他们对进程信息的获取还是通过TaskControlBlock

关于初始化信息补全

```rust
//fork()这里注意一下就行
UPSafeCell::new(TaskControlBlockInner {
    trap_cx_ppn,
    base_size: parent_inner.base_size,
    task_cx: TaskContext::goto_trap_return(kernel_stack_top),
    task_status: TaskStatus::Ready,
    memory_set,
    parent: Some(Arc::downgrade(self)),
    children: Vec::new(),
    exit_code: 0,
    heap_bottom: parent_inner.heap_bottom,
    program_brk: parent_inner.program_brk,
    // fork()的话记得从父进程那里把start_time和syscall_times继承下来
    start_time:parent_inner.start_time,
    syscall_times:parent_inner.syscall_times,
})
```

忘记记录了……之前的测例实现基本就是cv，注意放到正确的数据结构里面重新实现一次就行

遇到一个新问题

![image-20240506003949090](D:\116\sigs\my_OScamps_blog\一二阶段\rcore_lab.assets\image-20240506003949090-1716128810217-23.png)

![image-20240506004011708](D:\116\sigs\my_OScamps_blog\一二阶段\rcore_lab.assets\image-20240506004011708-1716128810217-24.png)

这个`Write`系统调用莫名其妙多出这么多次数

我决定在增加系统调用的方法中加一行调试，打印一下系统调用编号

```rust
/// 添加系统调用
pub fn add_current_syscall_times(&mut self,syscall_id:usize){
    let mut current_inner = self.current.as_mut().unwrap().inner_exclusive_access();
    current_inner.syscall_times[syscall_id] += 1;
    println!("{} + 1\n",syscall_id);
}
```

![image-20240506010214962](D:\116\sigs\my_OScamps_blog\一二阶段\rcore_lab.assets\image-20240506010214962-1716128810217-25.png)

出现了奇怪的输出

每次键入一个字符 对应着`read` `waitpid` `yield` `write`系统调用都+1了

虽然输出流打断了我的输入流 但是功能应该还是正常的 只是我没有键入回车键 所以用户程序没有被正常执行起来

应该是前面的实现有问题（

在父进程通过`fork()`系统调用创建子进程的时候，子进程不应该继承父进程的系统调用次数和开始时间

系统调用次数应该直接初始化为0才对（重新算

```rust
start_time:get_time_us() / 1000,
syscall_times:[0;MAX_SYSCALL_NUM]
```

ok 这里可以过了

现在又有新问题了 还是`ch3_taskinfo`的测例

![image-20240506011435160](D:\116\sigs\my_OScamps_blog\一二阶段\rcore_lab.assets\image-20240506011435160-1716128810217-26.png)

好像还是过不了 但是断言错误的次数确实是减少了……

好像是用`println!()`打印调试信息的问题，如果去掉的话`Write`系统调用的次数就不会增加

![image-20240506013300058](D:\116\sigs\my_OScamps_blog\一二阶段\rcore_lab.assets\image-20240506013300058-1716128810217-27.png)

还真是 回头看了一下`console.rs`里面对`println!()`的实现 很明显是基于`Write`的

不然平白无故你的OS怎么能打印东西的……把这事情给忘记了    ~~笑~~

先一次性把时间信息都打印下来吧 后面就不看了

![image-20240506013635316](D:\116\sigs\my_OScamps_blog\一二阶段\rcore_lab.assets\image-20240506013635316-1716128810217-28.png)

我切换分支到`ch4`重新跑一下这个用例 `t1=43` `t2=544` `t3=544` `info.time=501`是没有问题的 

睡了 明天再说



……睡觉的时候突然想到Lab5的`run_tasks()`方法应该是没有修改 所以没有把时间信息记录下来

没错 就是在此处补一个记录时间的功能就ok了

```rust
pub fn run_tasks(){
	loop{
		……
        if task_inner.start_time == 0{
                task_inner.start_time = get_time_us();
        }
        // taskinfo里面对taskinfo.time的计算除了1000(得到的是毫秒)
        // 这里不用除以1000了
        ……
	}
}
```



### Spawn系统调用实现

一遍过 感觉还是比较简单的…… 主要就是`fork()` `new()` 还有`exec()`的仿写

注意对parent字段特殊处理

`spawn`出来的进程的父进程应该为当前运行的进程

```rust
pub fn sys_spawn(path: *const u8) -> isize {
    trace!(
        "kernel:pid[{}] sys_spawn",
        current_task().unwrap().pid.0
    );
    let token = current_user_token();
    let path = translated_str(token, path);
    // 这里得到的data和sys_exec系统调用是类似的 可以直接作为参数被from_elf方法解析
    if let Some(data) = get_app_data_by_name(path.as_str()) {
        let task = current_task().unwrap();
        let new_task = task.spawn(data);
        let new_task_pid = new_task.pid.0;
        add_task(new_task);
        new_task_pid as isize
    }else{
        -1
    }
}

/// spawn基本就是对new fork还有exec三个方法的结合还有仿写
pub fn spawn(&self,elf_data: &[u8]) -> Arc<Self>{
    // memory_set with elf program headers/trampoline/trap context/user stack
    let (memory_set, user_sp, entry_point) = MemorySet::from_elf(elf_data);
    let trap_cx_ppn = memory_set
        .translate(VirtAddr::from(TRAP_CONTEXT_BASE).into())
        .unwrap()
        .ppn();
    // 给新进程分配一个进程号 分配内核栈
    let pid_handle = pid_alloc();
    let kernel_stack = kstack_alloc();
    let kernel_stack_top = kernel_stack.get_top();

    // 这里要特别处理一下parent字段 新创建的进程之父为当前正在运行的进程
    let parent = current_task().unwrap();

    let task_control_block = Arc::new(TaskControlBlock{
        pid: pid_handle,
        kernel_stack,
        inner:unsafe {
            UPSafeCell::new(TaskControlBlockInner {
                trap_cx_ppn,
                base_size: user_sp,
                task_cx: TaskContext::goto_trap_return(kernel_stack_top),
                task_status: TaskStatus::Ready,
                memory_set,
                parent: Some(Arc::downgrade(&parent)),
                children: Vec::new(),
                exit_code: 0,
                heap_bottom: user_sp,
                program_brk: user_sp,
                start_time:get_time_us(),
                syscall_times:[0;MAX_SYSCALL_NUM]
            })
        },
    });
    // 这里也是一样 在创建完子进程之后需要维护父进程的孩子列表
    let mut parent_inner = parent.inner_exclusive_access();
    parent_inner.children.push(task_control_block.clone());

    // TrapContext的逻辑应该是和exec还有new一样 准备一个全新的才行
    let trap_cx = task_control_block.inner_exclusive_access().get_trap_cx();
    *trap_cx = TrapContext::app_init_context(
        entry_point,
        user_sp,
        KERNEL_SPACE.exclusive_access().token(),
        kernel_stack_top,
        trap_handler as usize,
    );
    task_control_block
}
```



### Stride调度算法

#### 溢出问题

| A.stride(实际值) | A.stride(理论值) | A.pass(=BigStride/A.priority) |
| ---------------- | ---------------- | ----------------------------- |
| 65534            | 65534            | 100                           |
| B.stride(实际值) | B.stride(理论值) | B.pass(=BigStride/B.priority) |
| 65535            | 65535            | 50                            |

此时应该选择A作为调度的进程，而在一轮调度后，队列将如下：

| A.stride(实际值) | A.stride(理论值) | A.pass(=BigStride/A.priority) |
| ---------------- | ---------------- | ----------------------------- |
| 98               | 65634            | 100                           |
| B.stride(实际值) | B.stride(理论值) | B.pass(=BigStride/B.priority) |
| 65535            | 65535            | 50                            |

可以看到由于溢出的出现，进程间stride的理论比较和实际比较结果出现了偏差。我们首先在理论上分析这个问题：令`PASS_MAX`为当前所有进程里最大的步进值。则我们可以证明如下结论：对每次Stride调度器的调度步骤中，有其最大的步进值`STRIDE_MAX`和最小的步进值`STRIDE_MIN`之差：

```
STRIDE_MAX – STRIDE_MIN <= PASS_MAX
```

有了该结论，在加上之前对优先级有`Priority > 1`限制，我们有`STRIDE_MAX – STRIDE_MIN <= BIG_STRIDE`,于是我们只要将BigStride取在某个范围之内，即可保证对于任意两个 Stride 之差都会在机器整数表示的范围之内。而我们可以通过其与0的比较结构，来得到两个Stride的大小关系。在上例中，**虽然在直接的数值表示上 98 < 65535，但是 98 - 65535 的结果用带符号的 16位整数表示的结果为99,与理论值之差相等。**所以在这个意义下 98 > 65535。基于这种特殊考虑的比较方法，即便Stride有可能溢出，我们仍能够得到理论上的当前最小Stride，并做出正确的调度决定。

#### 实现流程

首先是在TCB里面增加进程优先级的字段`priority`和步长调度的参考数据`stride`

其实是对初始化信息的补全

一开始我是看到的`processor.rs`的`run_tasks()`模块，里面有一个`fetch_tasks()`的过程，取得目前应该运行的任务。但是`fetch_tasks()`是局限在`Task Manager`内部，缺少`Processor`结构，也就是当下的任务状态拿不到（就是有一种可能性是当下正在运行的进程`stride`还是最小）

然后继续看源码 

感觉这几个函数之间的关系有点懵

```rust
///The main part of process execution and scheduling
///Loop `fetch_task` to get the process that needs to run, and switch the process through `__switch`
pub fn run_tasks() {
    loop {
        let mut processor = PROCESSOR.exclusive_access();
        if let Some(task) = fetch_task() {
            let idle_task_cx_ptr = processor.get_idle_task_cx_ptr();
            // access coming task TCB exclusively
            let mut task_inner = task.inner_exclusive_access();
            let next_task_cx_ptr = &task_inner.task_cx as *const TaskContext;
            task_inner.task_status = TaskStatus::Running;
            if task_inner.start_time == 0{
                task_inner.start_time = get_time_us();
            }
            // release coming task_inner manually
            drop(task_inner);
            // release coming task TCB manually
            processor.current = Some(task);
            // release processor manually
            drop(processor);
            unsafe {
                __switch(idle_task_cx_ptr, next_task_cx_ptr);
            }
        } else {
            warn!("no tasks available in run_tasks");
        }
    }
}

pub fn suspend_current_and_run_next() {
    // There must be an application running.
    let task = take_current_task().unwrap();

    // ---- access current TCB exclusively
    let mut task_inner = task.inner_exclusive_access();
    let task_cx_ptr = &mut task_inner.task_cx as *mut TaskContext;
    // Change status to Ready
    task_inner.task_status = TaskStatus::Ready;
    drop(task_inner);
    // ---- release current PCB

    // push back to ready queue.
    add_task(task);
    // jump to scheduling cycle
    schedule(task_cx_ptr);
}

///Return to idle control flow for new scheduling
pub fn schedule(switched_task_cx_ptr: *mut TaskContext) {
    let mut processor = PROCESSOR.exclusive_access();
    let idle_task_cx_ptr = processor.get_idle_task_cx_ptr();
    drop(processor);
    unsafe {
        __switch(switched_task_cx_ptr, idle_task_cx_ptr);
    }
}
```

后面感觉还是得在`add`位置实现，也就是`fetch`还是从队头把进程取出来，但是在`add`增加进程的时候维护所有进程的`stride`顺序

```rust
pub fn add(&mut self, task: Arc<TaskControlBlock>) {
    //self.ready_queue.push_back(task);  简单的RR调度 
    let task_inner = task.inner_exclusive_access();
    let stride = task_inner.stride;
    drop(task_inner);
    // 获取队列长度 整个队列从头开始遍历
    let len = self.ready_queue.len();
    for queue in 0..len{
        let task1 = self.ready_queue.get_mut(queue).unwrap();
        let stride1 = task1.inner_exclusive_access().stride;
        if stride < stride1 {
            self.ready_queue.insert(queue, task);
            return
        }
    }
    self.ready_queue.push_back(task)
}
```



## Lab6

### 之前的测例

首先就是要通过之前的测例 `sys_spawn`和之前会有一些区别

主要就是获得程序数据的方式有差异

```rust
pub fn sys_spawn(path: *const u8) -> isize {
    trace!(
        "kernel:pid[{}] sys_spawn",
        current_task().unwrap().pid.0
    );
    let token = current_user_token();
    let path = translated_str(token, path);
    // 这里得到的data和sys_exec系统调用是类似的 可以直接作为参数被from_elf方法解析
    // 和ch5的区别主要就是在获得程序数据的方式 
    if let Some(app_inode) = open_file(path.as_str(), OpenFlags::RDONLY){
        let task = current_task().unwrap();
        let data = app_inode.read_all();
        let new_task = task.spawn(data.as_slice());
        let new_task_pid = new_task.pid.0;
        add_task(new_task);
        new_task_pid as isize
    }else {
        -1
    }
    // if let Some(data) = get_app_data_by_name(path.as_str()) {
    //     let task = current_task().unwrap();
    //     let new_task = task.spawn(data);
    //     let new_task_pid = new_task.pid.0;
    //     add_task(new_task);
    //     new_task_pid as isize
    // }else{
    //     -1
    // }
}
```

### 本实验测试点

![image-20240510225304112](D:\116\sigs\my_OScamps_blog\一二阶段\rcore_lab.assets\image-20240510225304112-1716128810217-29.png)

差最后一个

？？？

byd什么勾八

怎么实验还不能复现的 本地跑差一个点 在线ci全过了是吧

![image-20240510225444383](D:\116\sigs\my_OScamps_blog\一二阶段\rcore_lab.assets\image-20240510225444383-1716128810217-30.png)



## Lab8

`sys_enable_deadlock_detect`的逻辑如下

```rust
//需要给process_inner添加deadlock_detection_enabled字段
//表示死锁是否启用死锁检测相关的功能
pub fn sys_enable_deadlock_detect(is_enabled: usize) -> isize {
    trace!("kernel: sys_enable_deadlock_detect");
    let process = current_process();
    let mut process_inner = process.inner_exclusive_access();
    match is_enabled{
        0 => {
            process_inner.deadlock_detection_enabled = false;
            0 //成功禁用死锁检测算法
        },
        1 => {
            process_inner.deadlock_detection_enabled = true;
            0 //成功启用死锁检测算法
        }
        _ => -1 //输入其他参数 报错
    }
}
```

如果启用死锁检测功能的话，主要的检测就是在上锁相关的操作检测是否合法

这里有个困惑的点，就是一开始没搞懂`资源`到底是什么

其实就是各类的`lock`……能够得到锁 就代表得到了某个特定的资源



初始化是一门玄学……

需要自己设置好两个常量`MAX_THREADS`和`MAX_RESOURES`的数量，代表当前可以获得的资源

```rust
mutex_alloc:vec![None; MAX_THREADS],
mutex_request:vec![None; MAX_THREADS],
sem_alloc: vec![vec![0; MAX_RESOURCES]; MAX_THREADS],
sem_avail:vec![MAX_THREADS;MAX_RESOURCES],
sem_request:vec![None; MAX_THREADS],//每个线程都需要被匹配到 所以是MAX_THREADS
```



### 一些问题

![image-20240516200029337](D:\116\sigs\my_OScamps_blog\一二阶段\rcore_lab.assets\image-20240516200029337-1716128810217-31.png)

就卡在这里了，也不知道怎么回事

gpt问了一下

……死锁了 我就说为什么寄了



### 参考的实现

实在想不出来了 在网络上找了一个参考性的实现

发现他把死锁检测的逻辑封装成`ProcessLock`这个结构体里面……

也有看过其他的实现……做这个实验给我最大的感觉就是编程能力还是太弱了

同一个目的往往有不同的实现思路

在看一些参考的时候往往会被参考的代码困住，然后就陷入这个逻辑里面出不来了
