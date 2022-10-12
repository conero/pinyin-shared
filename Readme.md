## pinyin-shared

2020年8月17日



*基于 rust 编写实现以 `shared` 的方式编写，pinyin。*



### 编译

构建工具使用 **cargo**



cargo 工具使用

```shell
# 格式化代码
cargo fmt

# 执行测试
cargo test

# 编译
cargo build --release

# 运行指定包
cargo run --package cli

# 编译
cargo build --release --package cli
```



交叉编译

```shell
# 查看当前安装和支持的平台
rustup target list

# 编译时指定对应的目标环境
cargo build --release --target=x86_64-unknown-linux-musl
```







#### 结构

程序结构参照 [deno](https://github.com/denoland/deno) 项目风格



主要内容项：

- core               核心代码
- cli                    二进制命令行
- shared          dll/so 等文件库



#### 命令行

```shell
# 汉字转拼音
pyin.exe pyin <VALUE>
```







## 附录

### 参考

- [rust-pinyin](https://github.com/mozillazg/rust-pinyin)  参照学习该库（前期）

