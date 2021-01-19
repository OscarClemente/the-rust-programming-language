#!/bin/bash
# Compile Rust lib
cd hello_from_rust
cargo build
cd ..

# Compile C code
gcc -g main.c -o call_rust -lhello_from_rust -L./hello_from_rust/target/debug