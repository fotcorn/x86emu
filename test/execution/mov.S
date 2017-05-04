.text
.global _start
_start:

    mov $0xffffffffffffffff,%eax
    mov $0xffffffff, %rbx
    cmp %rax, %rbx
    jnz fail

    mov     $0,%rbx
    mov     $1,%rax
    int     $0x80

fail:
    int3
