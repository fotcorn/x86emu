.text
.global _start
_start:
    # 64bit with CL register
    mov $0x1234fdb512345678, %rax
    mov %rax, %rdx

    mov $0, %rcx
    shl %cl, %rax
    cmp %rax, %rdx
    jne fail
    jo fail
    jc fail
    jnp fail


    # 8bit with CL register
   /* mov $0x1234fdb512345678, %rax
    mov $0x1234fdb512345600, %rdx
    mov $8, %rcx
    shl %cl, %al
    cmp %rax, %rdx
    jne fail*/

    mov     $0,%rbx
    mov     $1,%rax
    int     $0x80

fail:
    int3
