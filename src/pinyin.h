// since: 2020-08-17
// author: Joshua Conero

#ifndef _PINYIN_H_
#define _PINYIN_H_

// @notice: 2020.08.19 使用 `extern` 指令代替
//__declspec 与 extern 的区别
#ifdef BUILDING_DLL
#define EXPORT __declspec(dllexport)
#else
#define EXPORT __declspec(dllimport)
#endif

// C++ 环境下
#ifdef __cplusplus
extern "C" {
#endif
    // 拼音结结构体
    struct PINYIN 
    {
        char *UTF_CODE;              //utf8 编好
        char *PY;                //实际的拼音，多音字使用`,`分割
        char *PY_ALPHA;          //拼音字母
        char *PY_ALPHA_TONE;     //拼音字母带数字音标
        char *WORD;              //汉字
    };

    // 文件导出
    // DLLIMPORT int add(int i,int j);

    // 获取文本的拼音
    char *pinyin(const char *text);
    // 获取文字的拼音
    char *word(const char *text);
    // 获取版本信息
    char *version();


#ifdef __cplusplus
};
#endif

#endif