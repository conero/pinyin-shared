/* Replace "dll.h" with the name of your header */
#include "demo.h"
#include <windows.h>

DLLIMPORT int add(int i, int j)
{
    return i+j;
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