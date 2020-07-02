# 跨平台编译

## 前置条件

- 已安装 `cargo-lipo` 以便生成 iOS 通用库

  ```sh
  $ cargo install cargo-lipo
  ```

- 已添加针对 iOS 的目标环境

  ```sh
  $ rustup target add aarch64-apple-ios x86_64-apple-ios
  ```

## 速览

1. 编译`.a`静态库
2. 自动生成`.h`头文件
3. 创建 Xcode 项目并集成运行

## 开始

### 1. 编译`.a`静态库

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

### 2. 自动生成`.h`头文件

安装`cbindgen`
```sh
cargo install --force cbindgen
```

生成`.h`头文件
```sh
cbindgen -l C -o header/ferris_says_hi.h
```

生成的`ferris_says_hi.h`
```c
#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

void fs_ferris_says_hi(void);
```

### 3. 创建 Xcode 项目并集成运行

新建项目：略

添加静态库依赖
1. 添加`target/universal/release/libferris_says_hi.a`到`Link Binary With Libraries`
2. 添加`target/universal/release`（静态库所在目录）到`Library Search Paths`
3. 添加`header/ferris_says_hi.h`到项目中，勾选`Copy item if need`

创建桥接文件并导入头文件

>  Tips: 任意新建 C 文件，提示生成桥接文件，随后删除，可减少手动配置过程

FerrisSaysHiApp-Bridging-Header.h
```c
#import "ferris_says_hi.h"
```

调用运行

ViewController.swift
```swift
    override func viewDidLoad() {
        super.viewDidLoad()

        fs_ferris_says_hi()
    }
```

Xcode 控制台输出：
```
 _______________________________
/ Hi，我是 Ferris，是 Rustacean \
\ 的好朋友，很高兴认识你~       /
 -------------------------------
        \
         \
            _~^~^~_
        \) /  o o  \ (/
          '_   -   _'
          / '-----' \

```

🎉 🎉 🎉 🎉 🎉

这样，Rust 的代码就顺利编译并运行到了iOS项目。可以留意到，目前只是一个无入参无出参的简单函数调用，接下来在此基础上，看看如何[进一步交互通信](./04-primitives-structs-enums.md)。