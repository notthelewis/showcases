package utils

import (
	"reflect"
	"unsafe"
)

func BytesToStringUNSAFE(in []byte) string {
    return *(*string)(unsafe.Pointer(&in))
}

func StringToBytesUNSAFE(in string) (b []byte) {
    sliceHeader := (*reflect.SliceHeader)(unsafe.Pointer(&b))
    strHeader := *(*reflect.StringHeader)(unsafe.Pointer(&in))

    sliceHeader.Data = strHeader.Data
    sliceHeader.Cap = strHeader.Len
    sliceHeader.Len = strHeader.Len

    return b
}
