# Rust 概览

## 代码初见

```rust
use std::io::{self, prelude::*, BufReader};
use std::net::TcpStream;
use std::str;

fn main() -> io::Result<()> {
    let mut tcp_stream = TcpStream::connect("127.0.0.1:8080")?;
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        tcp_stream.write(input.as_bytes())?;

        let mut buf_reader = BufReader::new(&tcp_stream);
        let mut buf: Vec<u8> = Vec::new();
        buf_reader.read_until(b'\n', &mut buf).expect("failed");

        let string = str::from_utf8(&buf).unwrap();
        println!("{}", string);
    }
}
```

Playground: https://play.rust-lang.org

## 简史

- 开发历史近 10 年，1.0 正式发布 5 年
- 起初是 Graydon 的编程语言实验项目
- 由 Mozilla 赞助主导，社区驱动

Rust 是现代化的年轻编程语言，语言特性上都借鉴了很多不同语言的优点。诞生于 Mozilla，2006 年 Graydon Hoare 开始开发， 2015 年发布的 1.0 版本，前不久过了个 [5 岁生日](https://twitter.com/rustlang/status/1261253754969640960)。Mozilla 开源背景和历史，以及在社区上的经验和积累是非常丰富的，因此 Rust 从最初就由良好的开源社区驱动，目标是成为安全可靠且高性能的系统级编程语言。既然是系统级语言，则对标的是 C 和 C++，无运行时和垃圾回收，确保性能的同时，能够方便地进行跨平台和嵌入式开发。

## 目标与设计哲学

- **实用**的系统级编程语言
- 同时追求**安全**、**并发**、**高性能**
- 显式高于隐式
- 运行时行为可预测

### 安全

* 类型系统

  * 编译器检查，暴露隐含错误
  * 编译器获取更多信息，利于优化
  * 增强可读性，代码表达性强

* 所有权系统

  > 每个被分配的内存都有一个独占其所有权的指针

* 借用和生命周期

  > 每个变量都有其生命周期，借用可通过标记生命周期供编译检查

### 零成本抽象

```ruby
// Ruby
6.times { put "6" }
```

```rust
// Rust
fn main() {
    6.times(|_| println!("6"));
}

pub trait Times where Self: Sized {
    fn times<F: FnMut(Self)>(&self, closure: F);
}

macro_rules! impl_times {
    ($ptype:ty) => {
        impl Times for $ptype {
            fn times<F: FnMut($ptype)>(&self, mut closure: F) {
                for i in 0..*self {
                    closure(i)
                }
            }
        }
    }
}

impl_times!(i8);
impl_times!(u8);
impl_times!(i16);
impl_times!(u16);
impl_times!(i32);
impl_times!(u32);
impl_times!(i64);
impl_times!(u64);
impl_times!(usize);
```

- 不存在运行时性能开销，编译时完成。
- 零成本抽象基石：泛型和 trait

### 实用性

* 友好的 FFI 支持，可以很好利用已有 C/C++ 等生态
* 包管理器 Cargo 及其一致的工作流
* 强大智能的编译器错误提示机制

> 学习 Rust 并把编译器当做「导师」，能帮助自己考虑更多可能的隐含问题，而不是「一把梭」。

## 跨平台特性







