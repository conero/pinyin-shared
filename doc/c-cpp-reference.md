# C/C++ 参照

> 2020.08.25
>
> Joshua Conero





## 指针/Pointer

### 变量

内存是一个存放数据的空间，**内存编址** 为内存进行的地址编码。内存是按一个字节接着一个字节的次序进行编址，每个字节都有个编号，我们称之为内存地址。



### 指针

指针也是一个变量，指向地址，属于复合类型。

*指针变量所存的内容就是内存的地址编号 ！*

二级指针，是一种指向指针的指针。

*所有类型的二级指针，由于均指向一级指针类型，一级指针类型大小是 4，所以二级指针的步长也是 4。*

数组名是一个指针常量.

