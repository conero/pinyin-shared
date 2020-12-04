import datetime

# 自动生成原始字符串
pyin_file = "../../doc/pinyin.txt"
targer_file = "./src/raw_dick.rs"

# 音调对照表
DICK = {
    'ue': ['üē', 'üé', 'üě', 'üè'],
    'a': ['ā', 'á', 'ǎ', 'à'],
    'e': ['ē', 'é', 'ě', 'è'],
    'i': ['ī', 'í', 'ǐ', 'ì'],
    'o': ['ō', 'ó', 'ǒ', 'ò'],
    'u': ['ū', 'ú', 'ǔ', 'ù'],
}

# 版本
gVersion = ''

# 拼音转字母
def pinyin_alpha(pyin):
    tone_alpha = pyin
    for ky in DICK:
        row = DICK[ky]
        for i in range(len(row)):
            tone = row[i]
            pyin = pyin.replace(tone, ky)
            tone_alpha = f'{pyin}{i + 1}'

    return pyin, tone_alpha


# 解析文件
def parse_line(raw, tag, callback):
    global gVersion
    with open(raw, encoding='utf8') as fp:
        for line in fp.readlines():
            line = line.strip()
            if line == "" or line.index("#") == 0:
                if 'version' in line:
                    queue =  line.split(':')
                    if len(queue) > 1:
                        gVersion = queue[1].strip()
                continue

            quque_str = line.split(":")
            if len(quque_str) > 1:
                code = quque_str[0].strip()
                quque_str = quque_str[1].strip().split("#")
                if len(quque_str) > 1:
                    pinyin = quque_str[0].strip()
                    word = quque_str[1].strip()
                    alpha, tone_alpha = pinyin_alpha(pinyin)
                    callback({'code': code, 'pinyin': pinyin, 'alpha': alpha, 'tone_alpha': tone_alpha, 'word': word})


# 生成 `C.h` 文件
def update_file(raw, tag):
    global gVersion
    now = datetime.datetime.now()
    buildTm = now.strftime("%Y-%m-%d %H:%M:%S")
    vlist = []

    def callback(line):
        # print(line)
        ss = f'{line["code"]}|{line["pinyin"]}|{line["alpha"]}|{line["tone_alpha"]}|{line["word"]}'
        vlist.append(ss)

    parse_line(raw, tag, callback)

    vLen = len(vlist)
    content = "{" + ";".join(vlist) + "}"
    content = f'''
// since: 2020-08-17
// author: Joshua Conero
// 结构体内容
// 原始数据字典，自动生成
pub const SOURCE_URL: &'static str = "https://github.com/mozillazg/pinyin-data";
pub const SOURCE_VERSION: &'static str = "{gVersion}";
pub const SOURCE_BUILD_DATE: &'static str = "{buildTm}";
pub const SOURCE_LENGTH: i32 = {vLen}; //字典长度
// `;`/ 换行符号， `|` 分割
//顺序: `code|pinyin|alpha|alpha_number|Chinese`
pub const SOURCE_TEXT: &'static str = r#"
{content}
"#;
    '''

    with open(tag, encoding='utf8', mode='w') as wf:
        wf.write(content)


if __name__ == "__main__":
    update_file(pyin_file, targer_file)
    print("文档已生成")
