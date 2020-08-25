//
// Created by 11066 on 2020/8/25.
// 变量测试
#include <iostream>
#include <stdlib.h>
using namespace std;

// `&` 取地址
int main(int argc, char **args){
    //基本类型测试
    bool isFake = true;
    char c = 'C';                   // char 一个单字符
    const char vc = 'J';            // char 一个单字符
    char ss[] = "Joshua Conero";    // char[] 字符串
    char *ps = ss;
    cout << "int) args: " << argc << ", &args:" << &argc << endl;
    cout << "bool) value: " << isFake << ", &address:" << &isFake << endl;
    cout << "char) value: " << c << ", &address:" << &c << endl;
    cout << "const char) value: " << vc << ", &address:" << &vc << endl;
    cout << "char[]) value: " << ss << ", &address:" << &ss << endl;

    //编译错误，ss 值无法修改。
    //ss = "With a long long time ago, thing start happen...";
    //ss = "With a long l";
    //cout << "char[]) value: " << ss << ", &address:" << &ss << endl;

    //指针（看做容器）
    int *pInt = &argc;
    int **ppi = &pInt;
    cout << "int pointer) pInt: " << pInt << ", *pInt :" << *pInt << endl;
    cout << "int pointer) pInt: " << ppi << ", *pInt :" << *ppi << ", **pInt :" << **ppi << endl;
    cout << "char pointer) pChar: " << ps << ", *pInt :" << *ps << endl;

    //字符串
    //char *psv1;   //输出前必须赋值
    char *psv1 = NULL;
    // `psv1 = "With a long long time ago, thing start happen...";` 直接赋值报错: ISO C++ forbids converting a string constant to 'char*'
    psv1 = (char *)"With a long long time ago, thing start happen...";
    cout << "char pointer) psv1: " << psv1 << ", *psv1 :" << *psv1 << endl;
    psv1 = (char *)"Suck is a fucking thing, ye."; //可修改值
    cout << "char pointer) psv1: " << psv1 << ", *psv1 :" << *psv1 << endl;

    //修改值
    *pInt = 100;
    cout << endl << "int pointer) change pointer value." << endl;
    cout << "int pointer) pInt: " << pInt << ", *pInt :" << *pInt << endl;
    cout << "int pointer) pInt: " << ppi << ", *pInt :" << *ppi << ", **pInt :" << **ppi << endl;

    system("pause");
    return 0;
}

