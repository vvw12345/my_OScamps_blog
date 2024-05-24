# Embassy中文文档

## 从裸机到异步RUST

### PAC

外设访问包(Peripheral Access Crate)

 在不直接访问内存的情况下 访问外设和寄存器最底层的API

```rust
#![no_std]
#![no_main]

use pac::gpio::vals;
use {defmt_rtt as _, panic_probe as _, stm32_metapac as pac};

#[cortex_m_rt::entry]
fn main() -> ! {
    // Enable GPIO clock
    // RCC:Reset and Clock Control  管理所有外设的时钟
    // 使得开发者可以开启或关闭外设的时钟
    let rcc = pac::RCC;
    unsafe {
        // AHB2是一条总线 
        // ahb2enr()方法用于操纵AHB2 Enable Register 使能
        // 该寄存器主要用于控制连接到AHB2总线的外设的时钟
        // 在该寄存器上，每一个外设有一个相应的位 
        rcc.ahb2enr().modify(|w| {
            // 启用gpiob和gpioc的时钟
            //GPIO:General Purpose Input/Output 通用输入/输出端口
            // gpiob和gpioc是GPIO的标签名称 用于区分不同的GPIO分组
            w.set_gpioben(true);
            w.set_gpiocen(true);
        });
		
        // 复位gpiob和gpioc 然后清除掉复位状态
        rcc.ahb2rstr().modify(|w| {
            // 复位是将所有寄存器和状态都恢复到初始状态
            w.set_gpiobrst(true);
            w.set_gpiocrst(true);
            // 清除复位状态代表复位已经完成 可以接收正常操作了
            w.set_gpiobrst(false);
            w.set_gpiocrst(false);
        });
    }

    // Setup button  配置GPIOC作为按钮输入
    let gpioc = pac::GPIOC;
    const BUTTON_PIN: usize = 13;
    unsafe {
        //设置按钮引脚为上拉
        //获取pupdr寄存器 随后修改
        gpioc.pupdr().modify(|w| w.set_pupdr(BUTTON_PIN, vals::Pupdr::PULLUP));
        //设置按钮引脚为推挽输出类型
        gpioc.otyper().modify(|w| w.set_ot(BUTTON_PIN, vals::Ot::PUSHPULL));
        //设置按钮引脚为输入模式
        gpioc.moder().modify(|w| w.set_moder(BUTTON_PIN, vals::Moder::INPUT));
    }

    // Setup LED 配置GPIOB作为LED灯输出
    let gpiob = pac::GPIOB;
    const LED_PIN: usize = 14;
    unsafe {
        // 设置LED引脚为浮动模式
        gpiob.pupdr().modify(|w| w.set_pupdr(LED_PIN, vals::Pupdr::FLOATING));
        // 设置LED引脚为推挽输出类型
        gpiob.otyper().modify(|w| w.set_ot(LED_PIN, vals::Ot::PUSHPULL));
        // 设置LED引脚为输出模式
        gpiob.moder().modify(|w| w.set_moder(LED_PIN, vals::Moder::OUTPUT));
    }

    // Main loop
    // 检查按钮状态 如果按下就点亮LED 否则熄灭LED
    // 不断轮询 忙等待
    loop {
        unsafe {
            if gpioc.idr().read().idr(BUTTON_PIN) == vals::Idr::LOW {
                gpiob.bsrr().write(|w| w.set_bs(LED_PIN, true));
            } else {
                gpiob.bsrr().write(|w| w.set_br(LED_PIN, true));
            }
        }
    }
}
```



### HAL

Hardware Abstraction Layer

用于模拟特定系统平台的细节使程序可以直接访问硬件的资源。将硬件方面的不同抽离OS的核心，核心模式的代码就不必因为硬件的不同而需要修改。因此硬件抽象层可加大软件的移植性。

```rust
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embassy_stm32::gpio::{Input, Level, Output, Pull, Speed};
use {defmt_rtt as _, panic_probe as _};

#[entry]
fn main() -> ! {
    // 初始化STM32硬件，使用默认配置
    let p = embassy_stm32::init(Default::default());
    
    // 创建一个新的GPIO输出实例，用于控制连接到PB14引脚的LED
    // LED初始状态为高电平（打开），并设置为非常高的速度
    // Speed 这个速度指的主要是信号边缘跳变的速度
    let mut led = Output::new(p.PB14, Level::High, Speed::VeryHigh);
    
    // 创建一个新的GPIO输入实例，用于读取连接到PC13引脚的按钮
    // 配置按钮引脚为上拉模式，以避免浮空状态
    let button = Input::new(p.PC13, Pull::Up);
	
    // 忙等检查按钮状态 并设置相应的LED灯的输出状态
    loop {
        if button.is_low() {
            led.set_high();
        } else {
            led.set_low();
        }
    }
}
```



### 中断

```rust
fn main() -> ! {
    // 初始化Embassy库，配置STM32硬件
    let p = embassy_stm32::init(Default::default());
    
    // 初始化LED为低电平和低速
    let led = Output::new(p.PB14, Level::Low, Speed::Low);
    // 初始化按钮输入，设置为上拉
    let mut button = Input::new(p.PC13, Pull::Up);

    // 使用Cortex-M的中断功能安全配置全局变量
    cortex_m::interrupt::free(|cs| {
        // 对按钮中断使能
        enable_interrupt(&mut button);

        // 将LED和按钮对象放入全局变量
        LED.borrow(cs).borrow_mut().replace(Some(led));
        BUTTON.borrow(cs).borrow_mut().replace(Some(button));

        // 使能对应的中断
        unsafe { NVIC::unmask(pac::Interrupt::EXTI15_10) };
    });

    // 主循环中使CPU进入低功耗模式等待事件
    loop {
        cortex_m::asm::wfe(); // 等待事件指令
    }
}

// 定义中断处理函数
#[interrupt]
fn EXTI15_10() {
    // 再次使用中断功能安全访问全局变量
    cortex_m::interrupt::free(|cs| {
        // 取得按钮和LED的引用
        let mut button = BUTTON.borrow(cs).borrow_mut();
        let button = button.as_mut().unwrap();

        let mut led = LED.borrow(cs).borrow_mut();
        let led = led.as_mut().unwrap();

        // 检查是否为我们的按钮引起的中断（check_interrupt未提供，需自行实现）
        if check_interrupt(button) {
            // 根据按钮的状态设置LED
            if button.is_low() {
                led.set_high();
            } else {
                led.set_low();
            }
        }

        // 清除中断标志（clear_interrupt未提供，需自行实现）
        clear_interrupt(button);
    });
}
```



### 异步

```rust
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    let mut led = Output::new(p.PB14, Level::Low, Speed::VeryHigh);
    let mut button = ExtiInput::new(Input::new(p.PC13, Pull::Up), p.EXTI13);

    loop {
        button.wait_for_any_edge().await;//等待外部事件时被挂起
        if button.is_low() {
            led.set_high();
        } else {
            led.set_low();
        }
    }
}
```







## 异步RUST VS RTOS



