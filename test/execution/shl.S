.text
.global _start
_start:
    # shlw       a=1234fdb512345678 b=0000000000000001 r=1234fdb51234acf0 ccin=0000 cc=0884
    mov $0x1234fdb512345678, %rax
    mov $0x0000000000000001, %rcx
    mov $0x1234fdb51234acf0, %rdx
    shlw %cl, %ax
    jno fail
    jc fail

    mov     $0,%rbx
    mov     $1,%rax
    int     $0x80
fail:
    int3
