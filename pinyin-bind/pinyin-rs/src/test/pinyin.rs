use crate::pinyin::Pinyin;

#[test]
fn search_word() {
    let py = Pinyin::new();
    let wd = py.search_word(String::from("杨"));
    println!("{:?}", wd);
}

#[test]
fn search_words() {
    let mut py = Pinyin::new();
    let wd = py.search_words(String::from("中华人民共和国"));
    println!("{:?}", wd);
}
