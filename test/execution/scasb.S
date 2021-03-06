.text
.global _start
_start:

    # ####################
    # scasb without repeat
    # ####################

    cld # clear direction flag

    movq $string0, %rdi
    mov $0, %al
    scasb (%rdi), %al
    jne fail

    # check if %rdi was incremented
    movq $string0, %rsi
    inc %rsi
    cmp %rsi, %rdi
    jne fail

    movq $string0, %rdi
    mov $1, %al
    scasb (%rdi), %al
    je fail

    movq $string1, %rdi
    mov $0, %al
    scasb (%rdi), %al
    je fail

    movq $string1, %rdi
    mov $1, %al
    scasb (%rdi), %al
    jne fail

    # with direction flag
    std
    movq $string0, %rdi
    mov $0, %al
    scasb (%rdi), %al
    jne fail

    # check if %rdi was decremented
    movq $string0, %rsi
    dec %rsi
    cmp %rsi, %rdi
    jne fail

    # ####################
    # scasb with repeat
    # ####################

    cld # clear direction flag

    movq $string_ab0cd, %rdi
    mov $0, %al
    mov $5, %rcx
    repne scasb (%rdi), %al
    jne fail  # 0 byte was found
    cmp $2, %rcx  # 0 byte is at position 2
    jne fail

    movq $string_ab0cd, %rdi
    mov $1, %al
    mov $5, %rcx
    repne scasb (%rdi), %al
    je fail  # 0 byte was not found
    cmp $0, %rcx  # all positions have been checked, nothing found
    jne fail

    mov     $0,%rbx
    mov     $1,%rax
    int     $0x80

fail:
    int3

string0:
    .ascii "\0"

string1:
    .ascii "\1"

string_ab0cd:
    .ascii "ab\0cd"
