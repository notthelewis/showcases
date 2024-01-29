package utils

import (
	"reflect"
	"unsafe"
)

// The UNSAFE functions cannot be used if the returned data is to be modified in any way. A panic will occur.
func BytesToStringUNSAFE(in []byte) string {
	return *(*string)(unsafe.Pointer(&in))
}

// The UNSAFE functions cannot be used if the returned data is to be modified in any way. A panic will occur.
func StringToBytesUNSAFE(in string) (b []byte) {
	sliceHeader := (*reflect.SliceHeader)(unsafe.Pointer(&b))
	strHeader := *(*reflect.StringHeader)(unsafe.Pointer(&in))

	sliceHeader.Data = strHeader.Data
	sliceHeader.Cap = strHeader.Len
	sliceHeader.Len = strHeader.Len

	return b
}
