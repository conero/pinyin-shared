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