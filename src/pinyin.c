#include <string.h>
#include <stdio.h>
#include <windows.h>
#include "pinyin.h"
#include "pinyin_dick.h"
#include "version.h"


// 获取拼音值
// @todo 业务实现
char *pinyin(const char *text){
    int vLen = sizeof(PINYIN_DICK);
    for(int i=0; i<vLen; i++){
        struct PINYIN py = PINYIN_DICK[i];
        // NULL 导致，段错误(Segmentation fault)
        if(py.WORD == NULL || py.PY == NULL){
            continue;
        }
        //printf("%s(%s).%s VS %s.\r\n", py.UTF_CODE, py.PY_ALPHA, py.WORD, text);
        if (strcmp(py.WORD, text) == 0){
            return py.PY;
        }
    }
    return "";
}

//通用的拼音查询其
struct PINYIN __pinyin(struct PINYIN py){
    int vLen = sizeof(PINYIN_DICK);
    for(int i=0; i<vLen; i++){
        struct PINYIN row = PINYIN_DICK[i];
        int isMatch = 0;
        if (py.UTF_CODE != NULL && strcmp(py.UTF_CODE, row.PY_ALPHA) == 0){
            isMatch = 1;
        }
        if(isMatch == 0 && py.PY != NULL && strcmp(py.PY, row.PY) == 0){
            isMatch = 1;
        }
        if(isMatch == 0 && py.PY_ALPHA != NULL && strcmp(py.PY_ALPHA, row.PY_ALPHA) == 0){
            isMatch = 1;
        }
        if(isMatch == 0 && py.PY_ALPHA_TONE != NULL && strcmp(py.PY_ALPHA_TONE, row.PY_ALPHA_TONE) == 0){
            isMatch = 1;
        }
        if(isMatch == 0 && py.WORD != NULL && strcmp(py.WORD, row.WORD) == 0){
            isMatch = 1;
        }
        if(isMatch == 1){
            return row;
        }
    }
    struct PINYIN nullPy = {NULL, NULL, NULL, NULL, NULL};
    return nullPy;
}

//获取值
char *word(const char *text){
    struct PINYIN py = {NULL, NULL, NULL, NULL, (char *)text};
    if(py.UTF_CODE == NULL){
        return py.PY;
    }
    return "";
}

// 获取版本信息
char *version(){
    //@note 地址处理错误: Segmentation fault
    static char ss[150];
    sprintf(ss, "%s/%s", VERSION, RELEASE);
    return ss;
}

//windows dll
BOOL WINAPI DllMain(HINSTANCE hinstDLL,DWORD fdwReason,LPVOID lpvReserved)
{
    switch(fdwReason) {
        case DLL_PROCESS_ATTACH:
            break;
        case DLL_PROCESS_DETACH:
            break;
        case DLL_THREAD_ATTACH:
            break;
        case DLL_THREAD_DETACH:
            break;
    }
    /* Return TRUE on success, FALSE on failure */
    return TRUE;
}
