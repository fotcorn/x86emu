.text
.global _start
_start:
    # 32 bit
    mov $0x0001888c00012341, %rax
    mov $0x0001888c00012341, %rbx
    add %rax, %rbx
    jnp fail

    mov     $0,%rbx
    mov     $1,%rax
    int     $0x80

fail:
    int3
