use crate::dick_data::PY_DICKS;
use crate::Dk;

// 字典
pub struct Dick {}

pub enum DickSearchType {
    Utf8Code,
    Pinyin,
    Alpha,
    Word,
}

// 查找参数
fn find_eq(src: &str, target: &str) -> bool {
    if src == target {
        return true;
    }

    let limiter = ",";
    if target.contains(limiter) {
        let str_queue: Vec<&str> = target.split(limiter).collect();
        if str_queue.contains(&src) {
            return true;
        }
    }

    return false;
}

// 字典工具，`find_` 表完全匹配, `search_` 表搜索，可能等于
impl Dick {
    // 公共类型完全
    fn find_by_elem<'a>(value: &'a str, tp: &'a DickSearchType) -> Option<&'a Dk> {
        for dk in PY_DICKS {
            match tp {
                DickSearchType::Utf8Code => {
                    if find_eq(value, dk.u8) {
                        return Some(dk);
                    }
                }
                DickSearchType::Pinyin => {
                    if find_eq(value, dk.py) {
                        return Some(dk);
                    }
                }
                DickSearchType::Alpha => {
                    if find_eq(value, dk.al) {
                        return Some(dk);
                    }
                }
                DickSearchType::Word => {
                    if find_eq(value, dk.wd) {
                        return Some(dk);
                    }
                }
            }
        }

        return None;
    }

    // 获取多次请求
    fn find_by_multi_elem<'a>(values: &'a str, tp: &'a DickSearchType) -> Vec<Option<&Dk>> {
        let mut cols: Vec<Option<&Dk>> = Vec::new();
        for v in values.chars() {
            // let dk = Dick::find_by_elem(&format!("{}", &v).as_str(), tp);
            //let dk = Dick::find_by_elem(format!("{}", v).as_str(), tp);
            //let dk = Dick::find_by_elem(v.to_string().as_str(), tp);
            let vs = v.to_string();
            let dk = Dick::find_by_elem(&vs.as_str(), tp);
            println!("{}", dk.unwrap());
            //println!("{}", v);
            // @todo 程序错误
            let dk = Dick::find_by_elem("", tp);
            cols.push(dk);
        }
        cols
    }

    // 通过汉字完全匹配字典
    pub fn find_by_word(word: &str) -> Option<&Dk> {
        Dick::find_by_elem(word, &DickSearchType::Word)
    }

    // 根据 utf8_code 搜索字典
    pub fn find_by_utf8(code: &str) -> Option<&Dk> {
        Dick::find_by_elem(code, &DickSearchType::Utf8Code)
    }

    // 根据拼音搜索
    pub fn find_by_alpha(alpha: &str) -> Option<&Dk> {
        Dick::find_by_elem(alpha, &DickSearchType::Alpha)
    }

    pub fn find_by_words<'a>(words: &'a str, sep: &'a str) -> String {
        let cols = Dick::find_by_multi_elem(words, &DickSearchType::Word);
        let mut ss = String::new();
        for dk in cols {
            if !dk.is_none() {
                ss.push_str(&format!("{}{}", dk.unwrap().py, sep));
            }
        }
        ss
    }
}
