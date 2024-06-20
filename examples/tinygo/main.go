package main

import (
	"math"
	"unsafe"
)

//go:wasm-module env
//export atan2
func atan2(x, y float32) float32

//go:wasm-module env
//export time
func time() float32

func sqrt(v float32) float32 {
	return float32(math.Sqrt(float64(v)))
}

var FRAMEBUFFER = (*[320 * 240]byte)(unsafe.Pointer(uintptr(120)))

//export upd
func upd() {
	var i int
	for i < 320*240 {
		t := time() * 63.0
		x := float32(i%320 - 160)
		y := float32(i/320 - 120)
		d := float32(40000.0) / sqrt(x*x+y*y)
		u := atan2(x, y) * 512.0 / 3.141
		c := uint8((int(d+t*2.0) ^ int(u+t)) >> 4)
		FRAMEBUFFER[i] = c
		i++
	}
}

func main() {
}
