package main

import (
	"fmt"
	"syscall"
	"unsafe"
)

func main() {
	demo := syscall.NewLazyDLL("demo.dll")

	// add
	add := demo.NewProc("add")
	// demo.add(156, 99)
	ret, _, _ := add.Call(uintptr(156), uintptr(99))
	fmt.Println("add: ", ret)

	// 获取 string 字符串的值
	// version
	version := demo.NewProc("version")
	// demo.add(156, 99)
	u1, u2, _ := version.Call()
	fmt.Println("version: ", u1, ",", u2)
	fmt.Println(UintptrConvString(u1))

	// 获取 string 字符串的值
	// version
	test := demo.NewProc("test")
	// demo.add(156, 99)
	str := "Create From The Golang.(贵阳)"
	t1, t2, _ := test.Call(uintptr(unsafe.Pointer(&str)))
	// t1, t2, _ := test.Call(uintptr(unsafe.Pointer(syscall.StringBytePtr(str))))
	fmt.Println("test: ", t1, ",", t2)
	fmt.Println(UintptrConvString(t1))
}

// link: https://studygolang.com/topics/2723
// uintptr --> string
func UintptrConvString(result uintptr) string {
	p := (*byte)(unsafe.Pointer(result))
	data := make([]byte, 0)
	for *p != 0 {
		data = append(data, *p)
		result += unsafe.Sizeof(byte(0))
		p = (*byte)(unsafe.Pointer(result))
	}
	return string(data)
}
