.text
.global _start
_start:

    mov     $-5,%rax
    add     $5,%rax
    jz jump0
    int3
jump0:
    jnz fail

    mov     $5,%rax
    add     $5,%rax
    jnz jump1
    int3
jump1:
    jz fail

    mov     $0,%rbx
    mov     $1,%rax
    int     $0x80
fail:
    int3
