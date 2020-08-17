from ctypes import *


dll = windll.LoadLibrary('./demo.dll')


print(dll)


result = dll.add(1, 100)
print(result)
print(type(result))