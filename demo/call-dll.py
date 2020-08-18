from ctypes import *


dll = windll.LoadLibrary('./demo.dll')


print(dll)

# int
result = dll.add(1, 100)
print(result)
print(type(result))


# char *
version = dll.version()
print(version)
print(type(version))

print(string_at(version))



# @todo 报错: OSError: exception: access violation writing 0x000000006C184014
# char
vchar = bytes("Ohh, opps.!", "utf8")
# test = dll.test(vchar)
# test = dll.test(c_char_p(b"Ohh, opps.!"))
test = dll.test()
# test = dll.test(c_char_p(vchar))
# test = dll.test(b"Ohh, opps.!")
print(test)
print(type(test))

print(string_at(test))