# 执行 py 生成 拼音字典
echo "将生成，拼音字典头部"
cd .\doc
python update-piinyin-c.py

# 返回到根目录，编译dll
echo "文件编译"
cd ../
gcc -shared -o ./dist/pinyin.dll ./src/pinyin.c