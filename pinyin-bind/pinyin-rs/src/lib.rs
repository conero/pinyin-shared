// 2020.12.04
// Rust 语言版本 pinyin 生成器
pub const VERSION: &'static str = "0.0.1";
pub const SINCE: &'static str = "2020-12-04";
pub const AUTHOR: &'static str = "Joshua Conero";
pub const RELEASE: &'static str = "dev";
pub const PROJECT_CODE: &'static str = "pinyin";

#[no_mangle]
extern "C" fn test() {
    println!("{} {}/{}", PROJECT_CODE, VERSION, RELEASE)
}

// 项目配置文档
#[cfg(test)]
mod test;
