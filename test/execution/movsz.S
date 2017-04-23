.text
.global  _start
_start:

    mov     $5, %r8b
    movsbw  %r8b, %bx
    mov     $5, %cx
    cmp     %cx, %bx
    jnz     fail
    

    mov     $5, %r8b
    movsbl  %r8b, %ebx
    mov     $5, %ecx
    cmp     %ecx, %ebx
    jnz     fail
    

    mov     $5, %r8b
    movsbq  %r8b, %rbx
    mov     $5, %rcx
    cmp     %rcx, %rbx
    jnz     fail
    

    mov     $5, %ax
    movswl  %ax, %ebx
    mov     $5, %ecx
    cmp     %ecx, %ebx
    jnz     fail
    

    mov     $5, %ax
    movswq  %ax, %rbx
    mov     $5, %rcx
    cmp     %rcx, %rbx
    jnz     fail
    

    mov     $5, %eax
    movslq  %eax, %rbx
    mov     $5, %rcx
    cmp     %rcx, %rbx
    jnz     fail
    

    mov     $5, %r8b
    movzbw  %r8b, %bx
    mov     $5, %cx
    cmp     %cx, %bx
    jnz     fail
    

    mov     $5, %r8b
    movzbl  %r8b, %ebx
    mov     $5, %ecx
    cmp     %ecx, %ebx
    jnz     fail
    

    mov     $5, %r8b
    movzbq  %r8b, %rbx
    mov     $5, %rcx
    cmp     %rcx, %rbx
    jnz     fail
    

    mov     $5, %ax
    movzwl  %ax, %ebx
    mov     $5, %ecx
    cmp     %ecx, %ebx
    jnz     fail
    

    mov     $5, %ax
    movzwq  %ax, %rbx
    mov     $5, %rcx
    cmp     %rcx, %rbx
    jnz     fail
    

    mov     $-5, %r8b
    movsbw  %r8b, %bx
    mov     $-5, %cx
    cmp     %cx, %bx
    jnz     fail
    

    mov     $-5, %r8b
    movsbl  %r8b, %ebx
    mov     $-5, %ecx
    cmp     %ecx, %ebx
    jnz     fail
    

    mov     $-5, %r8b
    movsbq  %r8b, %rbx
    mov     $-5, %rcx
    cmp     %rcx, %rbx
    jnz     fail
    

    mov     $-5, %ax
    movswl  %ax, %ebx
    mov     $-5, %ecx
    cmp     %ecx, %ebx
    jnz     fail
    

    mov     $-5, %ax
    movswq  %ax, %rbx
    mov     $-5, %rcx
    cmp     %rcx, %rbx
    jnz     fail
    

    mov     $-5, %r8b
    movzbw  %r8b, %bx
    mov     $251, %cx
    cmp     %cx, %bx
    jnz     fail
    

    mov     $-5, %r8b
    movzbl  %r8b, %ebx
    mov     $251, %ecx
    cmp     %ecx, %ebx
    jnz     fail
    

    mov     $-5, %r8b
    movzbq  %r8b, %rbx
    mov     $251, %rcx
    cmp     %rcx, %rbx
    jnz     fail
    

    mov     $-5, %ax
    movzwl  %ax, %ebx
    mov     $65531, %ecx
    cmp     %ecx, %ebx
    jnz     fail
    

    mov     $-5, %ax
    movzwq  %ax, %rbx
    mov     $65531, %rcx
    cmp     %rcx, %rbx
    jnz     fail
    
    mov     $0,%rbx
    mov     $1,%rax
    int     $0x80
fail:
    int3
