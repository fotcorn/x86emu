.text
.global _start
_start:
    mov $5, %ax
    mov $1, %bl
    div %bl

    # div %bh
    # div %bx
    # div %ebx
    # div %rbx

    mov     $0,%rbx
    mov     $1,%rax
    int     $0x80

fail:
    int3
