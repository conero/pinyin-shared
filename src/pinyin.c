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
        if(py.UTF_WORD == NULL || py.UTF_PY == NULL){
            continue;
        }
        //printf("%s VS %s.\r\n", py.UTF_WORD, text);
        if (strcmp(py.UTF_WORD, text) == 0){
            return py.UTF_PY;
        }
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
