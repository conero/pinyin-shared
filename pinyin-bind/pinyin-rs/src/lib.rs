use crate::raw_dick as raw;
use std::ffi::{CStr, CString};

// #[macro_use]
extern crate libc;

// 2020.12.04
// Rust 语言版本 pinyin 生成器
pub const VERSION: &'static str = "0.0.1";
pub const SINCE: &'static str = "2020-12-04";
pub const AUTHOR: &'static str = "Joshua Conero";
pub const RELEASE: &'static str = "dev";
pub const PROJECT_CODE: &'static str = "pinyin";

pub mod pinyin;
pub mod raw_dick; // 数据字典 // 拼音处理

#[no_mangle]
extern "C" fn test() {
    println!("{} {}/{}", PROJECT_CODE, VERSION, RELEASE)
}

//
// 项目相关
// 项目版本号
#[no_mangle]
extern "C" fn version() -> *mut libc::c_char {
    let c_str_changed = CString::new(format!("{}/{}", VERSION, RELEASE)).unwrap();
    c_str_changed.into_raw()
}

// 拼音相关
// 项目相关
// 项目版本号
// @todo 查询待完成
#[no_mangle]
extern "C" fn pinyin_words(text: *const libc::c_char) -> *mut libc::c_char {
    let mut c_str = unsafe {
        assert!(!text.is_null());
        CStr::from_ptr(text).to_string_lossy().into_owned()
    };

    c_str.push_str(" -》 文件已接收，C/C++ 乱码");
    // 构造数据返回
    let c_str_changed = CString::new(c_str).unwrap();
    c_str_changed.into_raw()
}

//
// 字典相关数据
// 字典导入时间
#[no_mangle]
extern "C" fn source_date() -> *mut libc::c_char {
    let c_str_changed = CString::new(raw::SOURCE_BUILD_DATE).unwrap();
    c_str_changed.into_raw()
}

// 字典长度
#[no_mangle]
extern "C" fn source_len() -> libc::c_int {
    raw::SOURCE_LENGTH
}

// 字典版本号
#[no_mangle]
extern "C" fn source_version() -> *mut libc::c_char {
    let c_str_changed = CString::new(raw::SOURCE_VERSION).unwrap();
    c_str_changed.into_raw()
}

// 字典依赖地址
#[no_mangle]
extern "C" fn source_url() -> *mut libc::c_char {
    let c_str_changed = CString::new(raw::SOURCE_URL).unwrap();
    c_str_changed.into_raw()
}

// 字典原始文本
#[no_mangle]
extern "C" fn source_text() -> *mut libc::c_char {
    let c_str_changed = CString::new(raw::SOURCE_TEXT).unwrap();
    c_str_changed.into_raw()
}

// 项目配置文档
#[cfg(test)]
mod test;
