use crate::pinyin::Pinyin;

#[test]
fn search_word() {
    let py = Pinyin::new();
    let wd = py.search_word(String::from("杨"));
    println!("{:?}", wd);
}
