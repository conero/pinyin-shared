use crate::raw_dick;

// 实际的拼音查询逻辑
//拼音字典数据结构体
// #[derive(Debug, Copy, Clone)]
#[derive(Debug, Clone)]
pub struct PinyinDick {
    pub code: String,       //代码
    pub pinyin: String,     //拼音
    pub alpha: String,      //字母
    pub tone_alpha: String, //带音调的多音字
    pub word: String,       //汉字
}

// 拼音
#[derive(Debug)]
pub struct Pinyin {
    _v_dick: Vec<PinyinDick>,
}

impl Pinyin {
    // 拼音是实例化
    pub fn new() -> Pinyin {
        let py = Pinyin {
            _v_dick: Vec::new(),
        };
        py
    }

    // 文本仅仅解析
    fn _parse_text_one(&mut self) -> &mut Pinyin {
        if self._v_dick.len() == 0 {
            self.parse_text();
        }
        return self;
    }

    // 解析文本生成拼音字典
    pub fn parse_text(&mut self) -> &mut Pinyin {
        let lines = raw_dick::SOURCE_TEXT.split(";");
        for line in lines {
            let ln_split = line.split("|");
            let values: Vec<&str> = ln_split.collect();
            if values.len() > 4 {
                let dd = PinyinDick {
                    code: values.get(0).unwrap().to_string(),
                    pinyin: values.get(1).unwrap().to_string(),
                    alpha: values.get(2).unwrap().to_string(),
                    tone_alpha: values.get(3).unwrap().to_string(),
                    word: values.get(4).unwrap().to_string(),
                };
                self._v_dick.push(dd);
            }
        }
        self
    }

    // 搜索字，全字典搜索，存在则终端搜索
    pub fn search_word(self, word: String) -> Option<PinyinDick> {
        let lines = raw_dick::SOURCE_TEXT.split(";");
        for line in lines {
            let ln_split = line.split("|");
            let values: Vec<&str> = ln_split.collect();
            if values.len() > 4 {
                let ref_word = values.get(4).unwrap().to_string();
                if ref_word.contains(&word) {
                    let dd = PinyinDick {
                        code: values.get(0).unwrap().to_string(),
                        pinyin: values.get(1).unwrap().to_string(),
                        alpha: values.get(2).unwrap().to_string(),
                        tone_alpha: values.get(3).unwrap().to_string(),
                        word: ref_word,
                    };
                    return Some(dd);
                }
            }
        }
        None
    }

    // 句子搜索
    /*pub fn search_words(&mut self, words: String) -> Option<Vec<PinyinDick>> {
        let words_split = words.split("");
        self._parse_text_one();
        let mut py_queue: Vec<PinyinDick> = Vec::new();
        for word in words_split {
            for py in &self._v_dick {
                if py.word.contains(&String::from(word)) {
                    py_queue.push(*py);
                }
            }
        }

        if py_queue.len() > 0 {
            return Some(py_queue);
        }
        None
    }*/
}
