/* Replace "dll.h" with the name of your header */
#include <windows.h>
#include <string.h>
#include "demo.h"

// 整形
DLLIMPORT int add(int i, int j)
{
    return i+j;
}

// 返回字符串
DLLIMPORT char *version(){
    char *ver = "v0.0.1;古丞秋";
    return ver; 
}

// 返回字符串
DLLIMPORT char *test(char name){
    char *author = strcat(AUTHOR, " -> ");
    return strcat(author, &name);
}

BOOL WINAPI DllMain(HINSTANCE hinstDLL,DWORD fdwReason,LPVOID lpvReserved)
{
    switch(fdwReason)
    {
        case DLL_PROCESS_ATTACH:
        {
            break;
        }
        case DLL_PROCESS_DETACH:
        {
            break;
        }
        case DLL_THREAD_ATTACH:
        {
            break;
        }
        case DLL_THREAD_DETACH:
        {
            break;
        }
    }

    /* Return TRUE on success, FALSE on failure */
    return TRUE;
}