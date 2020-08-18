pyin_file = "./pinyin.txt"
targer_file = "../src/pinyin_dick.h"

# 音调对照表
DICK = {
    'ue': ['üē','üé','üě','üè'],
    'a': ['ā','á','ǎ','à'],
    'e': ['ē','é','ě','è'],
    'i': ['ī','í','ǐ','ì'],
    'o': ['ō','ó','ǒ','ò'],
    'u': ['ū','ú','ǔ','ù'],
}

# 拼音转字母
def pinyin_alpha(pyin):
    tone_alpha = pyin
    for ky in DICK:
        row = DICK[ky]
        for i in range(len(row)):
            tone = row[i]
            pyin = pyin.replace(tone, ky)
            tone_alpha = f'{pyin}{i+1}'
            
    return pyin, tone_alpha

# 解析文件
def parse_line(raw, tag, callback):
    with open(raw, encoding='utf8') as fp:
        for line in fp.readlines():
            line = line.strip()
            if line == "" or line.index("#") == 0:
                continue

            quque_str = line.split(":")
            if len(quque_str) > 1:
                code = quque_str[0].strip()
                quque_str = quque_str[1].strip().split("#")
                if len(quque_str) > 1:
                    pinyin = quque_str[0].strip()
                    word = quque_str[1].strip()
                    alpha, tone_alpha = pinyin_alpha(pinyin)
                    callback({'code': code, 'pinyin': pinyin, 'alpha': alpha, 'tone_alpha':tone_alpha, 'word': word})


# 生成 `C.h` 文件
def update_file(raw, tag):

    vlist = []
    def callback(line):
        # print(line)
        ss = f'"{line["code"]}", "{line["pinyin"]}", "{line["alpha"]}", "{line["tone_alpha"]}", "{line["word"]}"'
        ss = "  {"+ss+"}"
        vlist.append(ss)

    parse_line(raw, tag, callback)

    vlen = len(vlist)
    content = "{\n"+",\n".join(vlist)+"\n}"
    content = f'''
// since: 2020-08-17
// author: Joshua Conero

// 有python 脚本生产
// 拼音字典
#ifndef _PINYIN_DICK_H_
#define _PINYIN_DICK_H_

#include "pinyin.h"

// 结构体内容
PINYIN PINYIN_DICK[{vlen}] = 
{content}
;
    '''

    with open(tag, encoding='utf8', mode='w') as wf:
        wf.write(content)


if __name__ == "__main__":
    update_file(pyin_file, targer_file)