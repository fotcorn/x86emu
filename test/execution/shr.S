.text
.global _start
_start:
    
    mov $0x1e30276, %rax
    mov $0x34, %rcx
    shr    %cl,%rax
    and    $0xf,%eax
    mov $0x0, %rbx
    cmp %rax, %rbx
    jnz fail

    mov $0x1e30276, %rax
    mov $0x18, %rcx
    shr    %cl,%rax
    and    $0xf,%eax
    mov $0x1, %rbx
    cmp %rax, %rbx
    jnz fail

    mov $0x1e30276, %rax
    mov $0x14, %rcx
    shr    %cl,%rax
    and    $0xf,%eax
    mov $0xe, %rbx
    cmp %rax, %rbx
    jnz fail

    mov     $0,%rbx
    mov     $1,%rax
    int     $0x80

fail:
    int3
