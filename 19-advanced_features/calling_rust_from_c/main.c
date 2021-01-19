#include <stdio.h>
#include <stdint.h>

int main() {
    printf("Hello, I'm C. I will try to call Rust.\n");
    uint32_t var_from_rust = hello_from_rust();
    printf("I'm C and I've called Rust wow! By the way, Rust gave me this value %d.\n", var_from_rust);
}