#include <string.h>
#include <stdio.h>
#include "pinyin.h"
#include "pinyin_dick.h"
#include "version.h"




// 获取拼音值
// @todo 业务实现
DLLIMPORT char *pinyin(char *text){
    char *author = strcat(AUTHOR, " -> ");
    return strcat(author, text);
}

// 获取版本信息
DLLIMPORT char *version(){
    char *ss;
    sprintf(ss, "%s/%s", VERSION, RELEASE);
    return ss;
}