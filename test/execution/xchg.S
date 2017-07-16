.text
.global _start
_start:
    mov $1, %rax
    mov $5, %rbx
    mov $1, %rcx

    mov $0x1234fdb512345678,%rax
    mov %rax, %rcx
    mov $0xfbcadd99fbca7654, %rbx
    mov %rbx, %rdx

    xchg %rax, %rbx

    cmp %rax, %rdx
    jne fail

    cmp %rbx, %rcx
    jne fail

    mov     $0,%rbx
    mov     $1,%rax
    int     $0x80

fail:
    int3
