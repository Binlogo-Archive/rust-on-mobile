# 针对移动跨平台的环境配置

## 前置环境

- Xcode
- 对 Rust 友好的[文本编辑器](https://www.rust-lang.org/tools)：VS Code/Sublime Text 3/Vim/Emacs/等
- 命令行工具

## Rust 环境配置

1. 下载 [rustup](https://www.rust-lang.org/tools/install)
```sh
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. 添加针对 iOS 编译的目标环境
```sh
$ rustup target add aarch64-apple-ios armv7-apple-ios armv7s-apple-ios x86_64-apple-ios
```
  - aarch64-apple-ios (64 位 ARM iOS，真机， 必选)
  - x86_64-apple-ios (64 位 x86 iOS, 模拟机，可选，推荐安装)
  - armv7-apple-ios （ARMv7 iOS，较旧真机，可选）
  - armv7s-apple-ios （ARMv7s iOS，较旧真机，可选）
  - i386-apple-ios（32 位 x86 iOS，模拟机， 可选，不推荐安装）
  附：[Rust 目标平台支持](https://forge.rust-lang.org/release/platform-support.html)

3. 安装 `cargo-lipo` 以便生成 iOS 通用库
```sh
$ cargo install cargo-lipo
```

## 确认环境是否配置完成

```sh
$ rustup show

Default host: x86_64-apple-darwin
rustup home:  /Users/WangXingbin/.rustup

installed toolchains
--------------------

stable-x86_64-apple-darwin
nightly-x86_64-apple-darwin (default)
1.39.0-x86_64-apple-darwin
1.41.0-x86_64-apple-darwin
1.42.0-x86_64-apple-darwin

installed targets for active toolchain
--------------------------------------

aarch64-apple-ios
armv7-apple-ios
armv7s-apple-ios
i386-apple-ios
x86_64-apple-darwin
x86_64-apple-ios

active toolchain
----------------

1.41.0-x86_64-apple-darwin
rustc 1.41.0 (5e1a79984 2020-01-27)
```

```sh
$ cargo lipo --version

Due to a known rustc issue, cargo-lipo can only be run on macOS. See https://github.com/rust-lang/rust/issues/36156#issuecomment-373201676 for more info.
cargo-lipo 2.0.0
```

### 快速创建并运行一个 Hello, World

```sh
$ cargo new hello && cd hello
    Created binary (application) `hello` package
$ cargo run
    Compiling hello v0.1.0 (/Users/WangXingbin/Desktop/rust_projects/hello)
    Finished dev [unoptimized + debuginfo] target(s) in 1.50s
     Running `target/debug/hello`
Hello, world!
```

Cargo 不仅是依赖管理工具，也是工作流工具，习惯 Cargo

## 更多参考

- [Install Rust](https://www.rust-lang.org/tools/install)