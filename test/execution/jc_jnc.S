.text
.global _start
_start:
    mov     $100,%rax
    mov     $50,%rbx
    sub     %rbx,%rax
    jnc     nc
    int3
nc:
    jc      fail

    mov     $0,%rbx
    mov     $1,%rax
    int     $0x80

fail:
    int3
