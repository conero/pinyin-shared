$isDebug = (($args -contains "-d") -or ($args -contains "--debug") -or ($args -contains "-g"));
echo "命令行参数: -d,--debug,-g 调式模式编译"


# 执行 py 生成 拼音字典
echo "将生成，拼音字典头部"
cd .\doc
python update-piinyin-c.py

# 返回到根目录，编译dll
echo "文件编译"
cd ../

if($isDebug){
    echo "pinyin.dll 调式模式编译。"
    gcc -shared -o ./dist/pinyin.dll ./src/pinyin.c -g
}else{
    gcc -shared -o ./dist/pinyin.dll ./src/pinyin.c
}
