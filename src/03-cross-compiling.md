# 跨平台编译

## 前置条件

- 已安装 `cargo-lipo` 以便生成 iOS 通用库

  ```sh
  $ cargo install cargo-lipo
  ```

## 速览

1. 编译`.a`静态库
2. 自动生成`.h`头文件
3. 创建 Xcode 项目并集成运行

## 开始

### 1\. 编译`.a`静态库

```sh
cargo lipo // 或者 cargo lipo --release
```

运行成功后，就可以在 `target/universal/{release|debug}/` 中找到生成的 `libferris_says_hi.a`静态库了。

查看下静态库的架构，可以看到包含了`x86_64`和`arm64`

```sh
$ lipo -info target/universal/debug/libferris_says_hi.a
Architectures in the fat file: target/universal/debug/libferris_says_hi.a are: x86_64 arm64
```

> cargo-lipo 命令做的就是将不同目标架构的生成包合成了，等价于`lipo -create`

### 2\. 自动生成`.h`头文件
