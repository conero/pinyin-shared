use crate::raw_dick as raw;
use std::ffi::CString;

// #[macro_use]
extern crate libc;

// 2020.12.04
// Rust 语言版本 pinyin 生成器
pub const VERSION: &'static str = "0.0.1";
pub const SINCE: &'static str = "2020-12-04";
pub const AUTHOR: &'static str = "Joshua Conero";
pub const RELEASE: &'static str = "dev";
pub const PROJECT_CODE: &'static str = "pinyin";

//拼音字典数据结构体
pub struct Pinyin {
    pub code: String,       //代码
    pub pinyin: String,     //拼音
    pub alpha: String,      //字母
    pub tone_alpha: String, //带音调的多音字
    pub word: String,       //汉字
}

// 数据字典
mod raw_dick;

#[no_mangle]
extern "C" fn test() {
    println!("{} {}/{}", PROJECT_CODE, VERSION, RELEASE);
}

// 字典导入时间
#[no_mangle]
extern "C" fn source_date() -> *mut libc::c_char {
    let c_str_changed = CString::new(raw::SOURCE_BUILD_DATE).unwrap();
    return c_str_changed.into_raw();
}

// 字典长度
#[no_mangle]
extern "C" fn source_len() -> libc::c_int {
    return raw::SOURCE_LENGTH;
}

// 字典版本
#[no_mangle]
extern "C" fn source_version() -> *mut libc::c_char {
    let c_str_changed = CString::new(raw::SOURCE_VERSION).unwrap();
    return c_str_changed.into_raw();
}

// 字典版本
#[no_mangle]
extern "C" fn source_url() -> *mut libc::c_char {
    let c_str_changed = CString::new(raw::SOURCE_URL).unwrap();
    return c_str_changed.into_raw();
}

// 项目配置文档
#[cfg(test)]
mod test;
