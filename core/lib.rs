pub mod dick;
pub mod dick_data;

// 拼音字典
// 使用简写的字母使其生成的模板文件
pub struct Dk {
    u8: &'static str, //utf8 编码
    py: &'static str, //音调拼音
    al: &'static str, //拼音字幕
    wd: &'static str, // 字
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
