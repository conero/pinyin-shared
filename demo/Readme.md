## dll 编写以及编译测试

2020年8月17日











### gcc

使用 gcc 编译如下（[参考](https://www.cnblogs.com/ser0632/p/4920653.html)）:

```powershell
# gcc -shared -o {name}.dll {name}.c
gcc -shared -o export.dll export.c
```

