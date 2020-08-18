#include <stdio.h>
// 获取命令行


int main(int argc,char *argv[]) {
    int count;
    printf("The command line has %d arguments :\n",argc-1);
    for (count = 1; count < argc; ++count) {
        printf("%d: %s\n",count,argv[count]);
    }

    return 0;
}