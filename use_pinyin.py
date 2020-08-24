from ctypes import *


dll = windll.LoadLibrary('./pinyin.dll')


print(dll)

# char *
version = dll.version()
print(version)
print(type(version))

print(string_at(version))



# @todo 报错: OSError: exception: access violation writing 0x000000006C184014
# @todo Error: OSError: exception: access violation reading 0x0000000064DFF000
# char
# vchar = bytes("杨", encoding="utf8")
vchar = bytes("杨", encoding="utf-8")
# test = dll.test(vchar)
# test = dll.test(c_char_p(b"Ohh, opps.!"))
test = dll.pinyin(vchar)
# test = dll.test(c_char_p(vchar))
# test = dll.test(b"Ohh, opps.!")
print(test)
print(type(test))

print(string_at(test))