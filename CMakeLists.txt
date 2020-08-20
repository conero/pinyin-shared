# CMake 构建文件
# 2020.08.20


#cmake最低版本需求，不加入此行会受到警告信息
CMAKE_MINIMUM_REQUIRED(VERSION 3.16)

#可创建多个项目
#项目名称
PROJECT(pinyin_share)
#编译 dll 动态链接库
ADD_LIBRARY(pinyin_share SHARED src/pinyin.c)

#项目名称
PROJECT(pinyin)
#生成应用程序 hello (在windows下会自动生成hello.exe)
ADD_EXECUTABLE(pinyin cli/pinyin.cpp cmd/command.cc)

#加载动态链接
TARGET_LINK_LIBRARIES(pinyin cmake-build-debug/libpinyin_dll.dll)






