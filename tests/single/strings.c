#include <stdio.h>
#include <string.h>
// 字符串测试

// 系统输入变量，测试
void test_args(int argc, char *argv[]){
    int count;
    printf("The command line has %d arguments :\n",argc-1);
    for (count = 1; count < argc; ++count) {
        // 尝试修改值
        char *cmd = argv[count];
        cmd = strcat(cmd, "(修改)");

        printf("%d: %s\n",count, cmd);
    }
}

// 字符串测试
void test_string(){
    char *c1 = "abc";
    char c2[] = "abc";      //数组无法修改
    int len;

    printf("初始值: %s, %s\n", c1, c2);

    strcat(c1, "*char");
    //c2 = strcat(c2, "char[]");
    //c2 = "cha";

    // Segmentation fault： 段错误就是指访问的内存超过了系统所给这个程序的内存空间
    /* 连接后，str1 的总长度 */
    len = strlen(c1);
    printf("strlen(str1) :  %d\n", c1);
    printf("修改后: %s, %s\n", c1, c2);
}

// 入口函数
int main(int argc,char *argv[]) {
    test_args(argc, argv);
    test_string();

    return 0;
}