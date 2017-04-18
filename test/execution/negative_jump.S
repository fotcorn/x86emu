.text
.global _start
_start:

    jmp next

end:
    mov     $0,%rbx
    mov     $1,%rax
    int     $0x80

next:
    jmp end
    int3
