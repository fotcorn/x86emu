.text
.global _start
_start:

    # shlb       a=1234fdb512345678 b=0000000000000001 r=1234fdb5123456f0 ccin=0000 cc=0085
    mov $0x1234fdb512345678, %rax
    mov $0x0000000000000001, %rcx
    mov $0x1234fdb5123456f0, %rdx
    shlb %cl, %al
    jno fail
    jc fail

    # shlb       a=8234fdb482345679 b=0000000000000008 r=8234fdb482345600 ccin=0000 cc=0044
    mov $0x8234fdb482345679, %rax
    mov $0x0000000000000008, %rcx
    mov $0x1234fdb512345600, %rdx
    shlb %cl, %al
    jnc fail
    jno fail

    # shlb       a=1234fdb512345678 b=0000000000000020 r=1234fdb512345678 ccin=0000 cc=0004
    mov $0x1234fdb512345678, %rax
    mov $0x0000000000000020, %rcx
    mov $0x1234fdb512345678, %rdx
    shlb %cl, %al
    jnc fail
    jnp fail
    jno fail

    mov     $0,%rbx
    mov     $1,%rax
    int     $0x80
fail:
    int3
