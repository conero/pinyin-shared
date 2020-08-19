#include <iostream>
#include "../src/pinyin.h"
#include "../cmd/lib.h"

using namespace std;

// notice: undefined reference to `__imp_version'  错误一般是由于dll链接调用错误导致的
// int main(int argc, char **argv)
// int main(int argc, char *argv[])
int main(int argc, char *argv[])
{
    cout << "Hello, world!" << endl;
    cout << version() << endl;
    cout << "Hello, world!" << endl;

    Command *cmd = new Command(argv);

    return 0;
}
