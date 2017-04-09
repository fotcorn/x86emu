.text
.global _start
_start:


    call func1
    mov %rax, %rbx
    call func2
    add %rbx,%rax

    mov $15,%rdx
    cmp %rax,%rdx
    jc fail
    cmp %rdx,%rax
    jc fail

    mov     $0,%rbx
    mov     $1,%rax
    int     $0x80

    int3
func1:
    mov $5, %rax
    ret
    int3

func2:
    mov $10, %rax
    ret
    int3

fail:
    int3
