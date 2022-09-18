use crate::dick_data::PY_DICKS;
use crate::Dk;

// 字典
pub struct Dick{
}

pub enum DickSearchType {
    Utf8Code,
    Pinyin,
    Alpha,
    Word
}

// 查找参数
fn find_eq(src: &str, target: &str) -> bool{
    if src == target{
        return true
    }

    let limiter = ",";
    if target.contains(limiter){
        let str_queue: Vec<&str> = target.split(limiter).collect();
        if str_queue.contains(&src){
            return true
        }

    }

    return false
}

// 字典工具，`find_` 表完全匹配, `search_` 表搜索，可能等于
impl Dick {
    // 公共类型完全
    fn find_by_elem(value: &str, tp: DickSearchType) -> Option<Dk> {
        for dk in PY_DICKS{
            match tp {
                DickSearchType::Utf8Code => {
                    if find_eq(value, dk.u8){
                        return Some(dk)
                    }
                }
                DickSearchType::Pinyin => {
                    if find_eq(value, dk.py){
                        return Some(dk)
                    }
                }
                DickSearchType::Alpha => {
                    if find_eq(value, dk.al){
                        return Some(dk)
                    }
                }
                DickSearchType::Word => {
                    if find_eq(value, dk.wd){
                        return Some(dk)
                    }
                }
            }
        }

        return None
    }

    // 通过汉字完全匹配字典
    pub fn find_by_word(word: &str) -> Option<Dk>{
        Dick::find_by_elem(word, DickSearchType::Word)
    }

    // 根据 utf8_code 搜索字典
    pub fn find_by_utf8(code: &str) -> Option<Dk>{
        Dick::find_by_elem(code, DickSearchType::Utf8Code)
    }

    // 根据拼音搜索
    pub fn find_by_alpha(alpha: &str) -> Option<Dk>{
        Dick::find_by_elem(alpha, DickSearchType::Alpha)
    }
}