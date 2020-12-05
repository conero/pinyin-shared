use crate::VERSION;

// 项目配置文档
#[cfg(test)]
mod pinyin;

#[test]
fn it_works() {
    println!("{}", VERSION);
    assert_eq!(2 + 2, 4);
}
