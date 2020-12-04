## Rust Bind

> 2020.12.04
>
> 使用 rust 生成 共享文件



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

