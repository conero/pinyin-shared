## pinyin-shared

2020年8月17日







*以 `shared` 的方式编写，pinyin。*



### 编译

构建工具使用 **cmake**

#### gcc

```shell
# cpp 编译，载入 dll，以及多文件编译
g++ .\cli\pinyin.cpp -o pinyin ./dist/pinyin.dll .\cmd\command.cc


# gdb c++/c 编译调试
# 如编译: build-with-g
gdb build-with-g.exe
# l
#(gdb): l
```



> gdb 常用命令

```
start                   #开始调试,停在第一行代码处,(gdb)start
l                       #list的缩写查看源代码,(gdb) l [number/function]
b <lines>           	#b: Breakpoint的简写，设置断点。(gdb) b 10
b <func>            	#b: Breakpoint的简写，设置断点。(gdb) b main
b filename:[line/function]  #b:在文件filename的某行或某个函数处设置断点
i breakpoints  			#i:info 的简写。(gdb)i breakpoints
d [bpNO]        		#d: Delete breakpoint的简写，删除指定编号的某个断点，或删除所有断点。断点编号从1开始递增。 (gdb)d 1
s                     	#s: step执行一行源程序代码，如果此行代码中有函数调用，则进入该函数；(gdb) s
n                     	#n: next执行一行源程序代码，此行代码中的函数调用也一并执行。(gdb) n
r                      	#Run的简写，运行被调试的程序。如果此前没有下过断点，则执行完整个程序；如果有断点，则程序暂停在第一个可用断点处。(gdb) r
c                       #Continue的简写，继续执行被调试程序，直至下一个断点或程序结束。(gdb) c
finish                	#函数结束
p [var]              	#Print的简写，显示指定变量（临时变量或全局变量 例如 int a）的值。(gdb) p a
display [var]           #display，设置想要跟踪的变量(例如 int a)。(gdb) display a
undisplay [varnum]     	#undisplay取消对变量的跟踪，被跟踪变量用整型数标识。(gdb) undisplay 1
set args               	#可指定运行时参数。(gdb)set args 10 20
show args           	#查看运行时参数。
q                       #Quit的简写，退出GDB调试环境。(gdb) q 
help [cmd]           	#GDB帮助命令，提供对GDB名种命令的解释说明。如果指定了“命令名称”参数，则显示该命令的详细说明；如果没有指定参数，则分类显示所有GDB命令，供用户进一步浏览和查询。(gdb)help
回车                    	#重复前面的命令，(gdb)回车

```



*gcc 编译乱码后科设置编码格式*

```powershell
#在编译参数中增加以下两条指令：

# -fexec-charset: 指定了字符串所使用的格式。
-fexec-charset=gbk

# -finput-charset: 用来指定输入文件的的字符编码，如果不指定，将无法将L“中文"这样的字符正确转化为宽字符。
-finput-charset=gbk
```







## 附录

### 参考

- [C/C++ 指针相接之基础篇](https://blog.csdn.net/weixin_39951988/article/details/87773322)
- [cmake](https://cmake.org/)
- [Segmentation Fault错误原因总结](https://blog.csdn.net/u010150046/article/details/77775114)
- [gdb 常用命令](https://blog.csdn.net/mercy_ps/article/details/81542986)
- [GCC编译环境中文乱码解决方案](https://www.cnblogs.com/CodeWorkerLiMing/p/12503166.html)

