.text
.global _start
_start:
    
    mov    $0x30, %rax
    mov    %ax,0xe(%rsp)

    mov    $0x31, %rax
    mov    %al,0xe(%rsp)

    movzbl  0xe(%rsp), %eax
    mov $0x31, %rbx
    cmp %rax, %rbx
    jnz fail

    movzbl  0xf(%rsp), %eax
    mov $0x0, %rbx
    cmp %rax, %rbx
    jnz fail

    mov     $0,%rbx
    mov     $1,%rax
    int     $0x80

fail:
    int3
