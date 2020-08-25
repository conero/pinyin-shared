//
// Created by 11066 on 2020/8/25.
// 变量测试
#include <string.h>
#include <stdio.h>
#include <stdlib.h>

// `&` 取地址
int main(int argc, char **args){
    //基本类型测试
    _Bool isFake = 1;
    char c = 'C';                   // char 一个单字符
    const char vc = 'J';            // char 一个单字符
    char ss[] = "Joshua Conero";    // char[] 字符串
    char *ps = ss;
    printf("int) args: %d, &args: %p.\n", argc, &argc);
    printf("bool) value: %d, &args: %p.\n", isFake, &isFake);
    printf("char) value: %c, &address: %p.\n", c, &c);
    printf("const char) value: %c, &address: %p.\n", vc, &vc);
    printf("const char) value: %s, &address: %p.\n", ss, &ss);

    //编译错误，ss 值无法修改。
    //ss = "With a long long time ago, thing start happen...";
    //ss = "With a long l";
    //cout << "char[]) value: " << ss << ", &address:" << &ss << endl;

    //指针（看做容器）
    int *pInt = &argc;
    int **ppi = &pInt;
    printf("int pointer) pInt: %d, &address: %p.\n", vc, &vc);
    printf("int pointer) pInt: %d, &address: %p, **pInt: %d.\n", ppi, *ppi, **ppi);
    printf("char pointer) pChar: %s, &address: %p.\n", ps, *ps);

    //字符串
    //char *psv1;   //输出前必须赋值
    char *psv1 = NULL;
    // `psv1 = "With a long long time ago, thing start happen...";` 直接赋值报错: ISO C++ forbids converting a string constant to 'char*'
    psv1 = "With a long long time ago, thing start happen...";
    printf("char pointer) psv1: %s, &*psv1: %p.\n", psv1, *psv1);
    psv1 = "Suck is a fucking thing, ye."; //可修改值
    printf("char pointer) psv1: %s, &*psv1: %p.\n", psv1, *psv1);

    //修改值
    *pInt = 100;
    printf("\nint pointer) change pointer value.\n");
    printf("int pointer) pInt: %d, &address: %p.\n", pInt, *pInt);
    printf("int pointer) pInt: %d, ppi: %p.\n", ppi, *pInt );


    //字符串判断
    char *psA = "Joshua", *psB = "Conero";
    if (strcmp(psA, psB) != 0){
        printf("%s != %s.\n", psA, psB);
    }
    psA = psB;
    if (strcmp(psA, psB) == 0){
        printf("%s == %s.\n", psA, psB);
    }
    //常量
    printf("strcmp(%s, BadGuy) = %d.\n", psA, strcmp(psA, "BadGuy"));
    printf("strcmp(%s, 西南) = %d.\n", psB, strcmp(psB, "西南"));

    system("pause");
    return 0;
}

