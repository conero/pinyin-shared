// since: 2020-08-17
// author: Joshua Conero

#ifndef _PINYIN_H_
#define _PINYIN_H_

#if BUILDING_DLL
#define DLLIMPORT __declspec(dllexport)
#else
#define DLLIMPORT __declspec(dllimport)
#endif

// 拼音结结构体
struct PINYIN 
{
    char *UTF_CODE;              //utf8 编好
    char *UTF_PY;                //实际的拼音，多音字使用`,`分割
    char *UTF_PY_ALPHA;          //拼音字母
    char *UTF_PY_ALPHA_TONE;     //拼音字母带数字音标
    char *UTF_WORD;              //汉字
};

// 文件导出
// DLLIMPORT int add(int i,int j);

// 后去文本的拼音
DLLIMPORT char *pinyin(char *text);
// 获取版本信息
DLLIMPORT char *version();

#endif