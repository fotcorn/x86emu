.text
.global  _start
_start:
    mov $0x0, %rax
    not %rax
    mov $0x5,%eax
    mov $0x5,%rbx
    cmp %rax,%rbx
    jnz fail

    mov     $0,%rbx
    mov     $1,%rax
    int     $0x80

fail:
    int3
    nop
