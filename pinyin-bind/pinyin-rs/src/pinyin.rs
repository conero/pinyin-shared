// 实际的拼音查询逻辑
//拼音字典数据结构体
pub struct Pinyin {
    pub code: String,       //代码
    pub pinyin: String,     //拼音
    pub alpha: String,      //字母
    pub tone_alpha: String, //带音调的多音字
    pub word: String,       //汉字
}
