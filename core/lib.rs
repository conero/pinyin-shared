use std::fmt;
use std::fmt::Formatter;

pub mod dick;
mod dick_data;

// 拼音字典
// 使用简写的字母使其生成的模板文件
pub struct Dk {
    pub u8: &'static str, //utf8 编码
    pub py: &'static str, //音调拼音
    pub al: &'static str, //拼音字幕
    pub wd: &'static str, // 字
}

impl fmt::Display for Dk {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(u8: {}, py: {}, al: {}, wd: {})",
            self.u8, self.py, self.al, self.wd
        )
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
