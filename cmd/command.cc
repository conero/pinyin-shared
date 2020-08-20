#include <iostream>
#include <map>
#include <string>
#include "lib.h"

using namespace std;

// /*******************************************  (Command/begin) *****************************************************/
// 构造函数
// 解析命令行参数
Command::Command(char *argv[]){
    this->_argv = argv;
    this->parse_argv();
}

// 解析参数
void Command::parse_argv(){
    int vLen = sizeof(this->_argv);
    if (vLen == 0){
        return;
    }
    
    // @todo notice: `Segmentation fault`
    for(int i=1; i<vLen; i++){
        char *arg = this->_argv[i];
        // 错误点: `Segmentation fault`
        cout << arg << endl;

        // @todo 数据简单的 demo
         if (i == 0){
             this->cc = arg;
         }else if (i == 1){
             this->suBcc = arg;
         }
    }
}

// /*******************************************  (Command/end) *****************************************************/




// /*******************************************  (Router/begin) *****************************************************/
Router::Router(Command *cmd){
    this->_cmd = cmd;
}


// 函数式命令注册
void Router::register_fn(string arg, CMD_CALLBACK_FN callback){
    this->_register_fn.insert(pair<string, CMD_CALLBACK_FN>(arg, callback));
}

// link: https://www.cnblogs.com/lifan3a/articles/8416929.html
// 执行命令行
void Router::run(){
    map<string, CMD_CALLBACK_FN>::iterator iter;
    map<string, CMD_CALLBACK_FN> vMap = this->_register_fn;
    iter = vMap.begin();
    while(iter != vMap.end()) {
        cout << iter->first << " : " << endl;
        iter++;
    }
}


// /*******************************************  (Router/end) *****************************************************/


