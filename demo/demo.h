#ifndef _DEMO_H_
#define _DEMO_H_

#if BUILDING_DLL
#define DLLIMPORT __declspec(dllexport)
#else
#define DLLIMPORT __declspec(dllimport)
#endif

// 定义变量
#define AUTHOR   "Joshua Conero"

DLLIMPORT int add(int i,int j);
DLLIMPORT char *version();
DLLIMPORT char *test(char name);

#endif