package main

/*
#cgo LDFLAGS: -L../rust/target/release -lrust_interop
#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

extern int32_t add(int32_t a, int32_t b);
extern uint8_t* create_proof(const uint8_t* preimage, size_t preimage_len);
extern bool verify_proof(uint8_t* proof, size_t proof_len);
*/
import "C"
import (
	"fmt"
	"unsafe"
)

func main() {
	preimage := make([]byte, 80)
	for i := range preimage {
		preimage[i] = 42
	}

	proofPtr := C.create_proof(
		(*C.uint8_t)(unsafe.Pointer(&preimage[0])),
		C.size_t(len(preimage)),
	)

	proof := C.GoBytes(unsafe.Pointer(proofPtr), C.int(48+96+48))

	fmt.Printf("Proof created: %x\n", proof)

	C.free(unsafe.Pointer(proofPtr))

	verification := C.verify_proof(
		(*C.uint8_t)(unsafe.Pointer(&proof[0])),
		C.size_t(len(proof)),
	)

	fmt.Printf("Proof verified: %v\n", verification)
}
