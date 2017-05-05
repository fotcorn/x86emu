#include <stdint.h>

void __puthex(uint64_t value);

int boot_params;

void __putstr(const char *s) {
    if (s[1] != 0) {
        __asm__("int3");
    }
}

void main(void) {
    __puthex(0x111111);
    __asm__("mov     $0,%rbx");
    __asm__("mov     $1,%rax");
    __asm__("int     $0x80");
}
