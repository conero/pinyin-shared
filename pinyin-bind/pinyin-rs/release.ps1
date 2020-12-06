#程序发布
cargo fmt
cargo rustc --lib --release

#程序日志
Copy-Item .\target\release\pinyin.dll .\dist\pinyin.dll

Write-Host "程序已编译成功"
Read-Host "请输入任意键结束"
#>