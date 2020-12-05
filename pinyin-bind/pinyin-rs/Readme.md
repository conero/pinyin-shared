## Rust Bind

> 2020.12.04
>
> 使用 rust 生成 共享文件



**说明**

将拼音字典 rust 字符串常量来解析拼音接口体。



### 起步



cargo 命令

[cargo fmt](https://github.com/rust-lang/rustfmt)

```shell
# 项目代码格式化
cargo fmt
```



### 共享文件库编译

需参考 Rust FFI 特性

- 配置 `Cargo.toml` 中 **lib** 项目

```toml
[lib]
crate-type = ["dylib"]
#项目的入口文件
path = "src/lib.rs"
```



- 使用 cargo进行比编译

```shell
# 编译命令
cargo rustc --lib --release

# 执行测试
cargo test
# 执行测试，且显示打印的内容
cargo test -- --nocapture

# 生成项目文档
rustdoc src/lib.rs
```



### 附录

#### 参考

- [Rust 中文文档](https://github.com/KaiserY/trpl-zh-cn)  github
- [Rust 中文文档---程序园](http://www.voidcn.com/course/project/rkllub)
- [cargo 等文档用户](https://github.com/zzy)
  - [cargo 中文文档](https://cargo.budshome.com/)
  - [Cargo 文档2](https://github.com/chinanf-boy/cargo-book-zh)
- rust 编译为 dll
  - [将Rust编译成库](http://www.voidcn.com/article/p-aezmifri-bnw.html)
  - [--crate-type does not override lib properties on manifest](https://github.com/rust-lang/cargo/issues/6160)
  - [rust-ffi](https://doc.rust-lang.org/book/ffi.html)
  - [rust-ffi-guide](https://michael-f-bryan.github.io/rust-ffi-guide/)
  - [Rust FFI 编程 - Rust导出共享库02](https://blog.csdn.net/u012067469/article/details/107551919)
- [国内 cargo 镜像](https://blog.csdn.net/setlilei/article/details/106204105)
- dll 测试
  - [python 读取 dll 函数错误 - OSError: exception: access violation reading 0x0000000000000001](https://blog.csdn.net/jacke121/article/details/79837632)

