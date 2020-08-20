# 调试
$isDebug = (($args -contains "-d") -or ($args -contains "--debug") -or ($args -contains "-g"));
$rbd = (($args -contains "-r") -or ($args -contains "--rbdll"));
echo "命令行参数: -d,--debug,-g 调式模式编译"
echo "命令行参数: -r,--rbdll    dll 库重编译"
echo ""

# 编译 dll
$dll = "./pinyin.dll"

#dll 不存在时，编译对应的文件
if(-not (Test-Path -Path $dll )){
    if($isDebug){
        ./build.ps1 -d
    }else{
        ./build.ps1
    }
    cp .\dist\pinyin.dll pinyin.dll
}elseif ($rbd) {
    echo "重编译，dll 库文件"
    rm pinyin.dll

    if($isDebug){
        ./build.ps1 -d
    }else{
        ./build.ps1
    }

    cp .\dist\pinyin.dll pinyin.dll
}


echo "编译 cli 的应用"

# 编译文件
if($isDebug)
{
    echo "带调试模式的输出"
    g++ .\cli\pinyin.cpp -o pinyin ./dist/pinyin.dll .\cmd\command.cc -fexec-charset=gbk -g
}else{
    g++ .\cli\pinyin.cpp -o pinyin ./dist/pinyin.dll .\cmd\command.cc -fexec-charset=gbk
}


