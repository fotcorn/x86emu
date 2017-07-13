.text
.global _start
_start:
    # 32 bit
    mov $5, %eax
    mov $2, %ebx
    mov $0, %edx
    div %ebx

    cmp $2, %eax
    jnz fail
    cmp $1, %edx
    jnz fail


    mov $6, %eax
    mov $2, %ebx
    mov $0, %edx
    div %ebx

    cmp $3, %eax
    jnz fail
    cmp $0, %edx
    jnz fail

    # 64 bit
    mov $5, %eax
    mov $2, %ebx
    mov $0, %edx
    div %ebx

    cmp $2, %eax
    jnz fail
    cmp $1, %edx
    jnz fail


    mov $6, %eax
    mov $2, %ebx
    mov $0, %edx
    div %ebx

    cmp $3, %eax
    jnz fail
    cmp $0, %edx
    jnz fail


    mov     $0,%rbx
    mov     $1,%rax
    int     $0x80

fail:
    int3
