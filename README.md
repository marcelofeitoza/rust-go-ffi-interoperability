## Rust-Go FFI

This project demonstrates an integration of **Rust** with **Go** using FFI (Foreign Function Interface). It involves a Zero-Knowledge Proof circuit implemented in Rust, which is called from a Go program to create and verify proofs.

### Overview

-   **Rust Code:** Implements a ZKP circuit using the Bellman crate, proof serialization/deserialization, and exposes functions to be called from Go, using C bindings.
-   **Go Code:** Calls Rust functions to create and verify proofs using the exposed FFI, using cgo (`Go's C Foreign Function Interface`).

### How FFI Works

#### Rust Side

To expose Rust functions to Go, we use `#[no_mangle]` to prevent Rust from changing the names of the functions and `extern "C"` to make them compatible with C calling conventions.

> `unsafe` blocks were used because raw pointers were used to pass data between Rust and Go.

**Example Rust FFI (in `ffi.rs`):**

```rust
#[no_mangle]
pub unsafe extern "C" fn create_proof(preimage: *const u8, preimage_len: usize) -> *mut u8 {
    // Function body
}

#[no_mangle]
pub unsafe extern "C" fn verify_proof(proof: *const u8, proof_len: usize) -> bool {
    // Function body
}
```

#### Go Side

The Go code uses `cgo` to import and call the Rust functions. It includes function prototypes to match the Rust functions and handles converting Go types to C-compatible types using `unsafe` and `C` pointers.

**What makes this work:**

These lines in the Go code include the Rust functions and declare their prototypes, and is placed below the `package main` declaration and above the `import "C"` line:

```go
package main

/*
#cgo LDFLAGS: -L../rust/target/release -lrust_interop
#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

// Declare the Rust function prototypes for cgo
extern uint8_t* create_proof(const uint8_t* preimage, size_t preimage_len);
extern bool verify_proof(uint8_t* proof, size_t proof_len);
*/
import "C"
```

In here we declared the "interface" to the Rust functions, so that Go can call them.

### Usage

1. **Build the Rust Library:**

    Navigate to the `rust` directory and run:

    ```bash
    cd ./rust && cargo build --release && cd ../
    ```

2. **Run the Go Program:**

    Navigate to the `go` directory and run:

    ```bash
    go run ./go/main.go
    ```
