# Rust 概览

## 代码初见

通过`Show me the code`快速直观地认识下 Rust 代码

Tips: [Playground](https://play.rust-lang.org) & [mdBook](https://github.com/rust-lang/mdBook)

### Hello, World

```rust
fn hello(name: Option<&str>) {
    match name {
        Some(name) => println!("Hello, {}!", name),
        None => println!("Hello, World!"),
    }
}

hello(None);
hello(Some("Binlogo"));
```

- 无 `null` 和 `void *` 等类型
- `Option` 枚举标识值的可空，编译器检查，避免运行时错误

### 翻转字符串

```rust
pub fn reverse(input: &str) -> String {
    input.chars().filter(|c| !c.is_whitespace()).rev().collect()
}
let s = "a1  b2  cdefg";
println!("{}", reverse(&s));
```

- 代码抽象程度高，表达能力足够强，且无运行时消耗（[零成本抽象](#零成本抽象)）
- 高阶函数支持

### 表示时间

```rust
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}
const HOURS_PER_DAY: i32 = 24;
const MINUTES_PER_HOUR: i32 = 60;
const MINUTES_PER_DAY: i32 = MINUTES_PER_HOUR * HOURS_PER_DAY;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = total_minutes(hours, minutes);
        let hours = total_minutes / MINUTES_PER_HOUR;
        let minutes = total_minutes % MINUTES_PER_HOUR;
        Self { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(self.hours, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

fn total_minutes(hours: i32, minutes: i32) -> i32 {
    let total_minutes = (hours * MINUTES_PER_HOUR + minutes) % MINUTES_PER_DAY;
    if total_minutes >= 0 {
        total_minutes
    } else {
        total_minutes + MINUTES_PER_DAY
    }
}

#[test]
fn test_on_the_hour() {
    assert_eq!(Clock::new(8, 0).to_string(), "08:00");
}

#[test]
fn test_midnight_is_zero_hours() {
    assert_eq!(Clock::new(24, 0).to_string(), "00:00");
}

#[test]
fn test_add_across_midnight() {
    let clock = Clock::new(23, 59).add_minutes(2);
    assert_eq!(clock.to_string(), "00:01");
}
```

- `derive` 运用宏

  ```rust
  impl<T: PartialEq> PartialEq for Clock {
    fn eq(&self, other: &Clock) -> bool {
        self.a == other.a && self.b == other.b
    }

    fn ne(&self, other: &Clock) -> bool {
        self.a != other.a || self.b != other.b
    }
  }
  ```

- `traits` 特性

- `structs` 结构体

- `cargo test`可快速进行运行测试

### TCP 的简单应用

Server:

```rust
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() -> io::Result<()> {
    let tcp_listener = TcpListener::bind("127.0.0.1:8080")?;
    let mut thread_vec: Vec<thread::JoinHandle<()>> = vec![];
    for stream in tcp_listener.incoming() {
        let stream = stream.expect("failed"); // Variable shadowing
        let handle = thread::spawn(|| {
            handle_client(stream).unwrap_or_else(|e| eprintln!("{:?}", e));
        });
        thread_vec.push(handle);
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let mut buf = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 {
            return Ok(());
        }
        let str = String::from_utf8(buf.to_vec());
        println!("{:?}", str.unwrap());
        stream.write(&buf[..bytes_read])?;
    }
}
```

Client

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

- TCP 网络编程，系统级编程语言
- 默认不可变，可变量须显示声明
- Variable shadowing
- 可恢复的错误传递`?`

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

- 类型系统

  - 编译器检查，暴露隐含错误
  - 编译器获取更多信息，利于优化
  - 增强可读性，代码表达性强

- 所有权系统

  > 每个被分配的内存都有一个独占其所有权的指针

- 借用和生命周期

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

- 友好的 FFI 支持，可以很好利用已有 C/C++ 等生态
- 包管理器 Cargo 及其一致的工作流
- 强大智能的编译器错误提示机制

> 学习 Rust 并把编译器当做「导师」，能帮助自己考虑更多可能的隐含问题，而不是「一把梭」。

## 跨平台特性

目前跨平台的编程语言以 C++ 为主，例如较为主流的 Facebook 的跨平台布局框架 Yoga，腾讯微信的跨平台网络库 mars。而 Rust 是一门对 FFI 良好支持的系统级语言，既能够和 Swift/Kotlin 等平台支持语言一样有良好的抽象和表达性，又能够达到 C/C++ 的性能的同时，还可以在编译期保证安全。因此，如果不是像腾讯一样有着大量 C++ 开发人员，又有跨平台的开发需求，那么 Rust 会是个不错的选择。

## 更多参考

- [PingCAP - Talent Plan](https://github.com/pingcap/talent-plan) - PingCAP 的 Rust 学习资源
- [Ferrous Teaching Material](https://github.com/ferrous-systems/teaching-material) - Rust 教学资源，非常全面
- [棒棒彬的第二大脑-编程语言-Rust](https://binlogo.github.io/Knowledge-Track/programming-languages/rust/rust.html)

附： 微软/Stack Overflow 尝试后对 Rust 的追捧热评
- [2020/06/05 - Stack Overflow - Why the developers who use Rust love it so much](https://stackoverflow.blog/2020/06/05/why-the-developers-who-use-rust-love-it-so-much/)
- [2020/06/02 - So What's Up with Microsoft's (and Everyone Else's) Love of Rust?](https://visualstudiomagazine.com/articles/2020/06/02/rust-love.aspx)
- [2020/01/20 - Stack Overflow - What is Rust and why is it so popular?](https://stackoverflow.blog/2020/01/20/what-is-rust-and-why-is-it-so-popular/)
