// c++ 命令行解析
#ifndef CMD_LIB_H
#define CMD_LIB_H

#include <map>
#include <string>

using namespace std;
// 命令解析行类
class Command
{
    public:
        string cc;
        string subcc;
        Command(char **argv);          //构造函数不等指定返回
        void parse_argv();             //命令行解析
    
    private:
        char **_argv;
};

// 函数式回调注册
typedef void (__stdcall *CMD_CALLBACK_FN)(const Command *cmd);


// 命令行路由
class Router 
{
    public:
        Router(Command *cmd);                                           //路由器初始化
        void register_fn(string arg, CMD_CALLBACK_FN callback);         //函数注册
        void run();                                                     //运行

    private:
        map<string, CMD_CALLBACK_FN> _register_fn;                      //函数
        Command *_cmd;
};

#endif