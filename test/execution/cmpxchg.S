.text
.global _start
_start:
    mov $1, %rax
    mov $5, %rbx
    mov $1, %rcx

    # positive outcome: rAX == destination
    cmpxchg %rbx, %rcx

    # zero flag should be set
    jnz fail

    # destination is set to source
    cmp $5, %rcx
    jne fail

    # rAX is unchanged
    cmp $1, %rax
    jne fail
    
    # source is unchanged
    cmp $5, %rbx
    jne fail


    # negative outcome: rAX != destination
    mov $1, %rax
    mov $5, %rbx
    mov $2, %rcx

    cmpxchg %rbx, %rcx

    # zero flag should not be set
    jz fail

    # destination is unchanged
    cmp $2, %rcx
    jne fail

    # rAX is set to destination
    cmp $2, %rax
    jne fail

    # source is unchanged
    cmp $5, %rbx
    jne fail

    mov     $0,%rbx
    mov     $1,%rax
    int     $0x80

fail:
    int3
