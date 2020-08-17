// since: 2020-08-17
// author: Joshua Conero

#ifndef _PINYIN_H_
#define _PINYIN_H_

// 拼音结结构体
struct PINYIN 
{
    char UTF_CODE;              //utf8 编好
    char UTF_PY;                //实际的拼音，多音字使用`,`分割
    char UTF_PY_ALPHA;          //拼音字母
    char UTF_WORD;              //汉字
};


