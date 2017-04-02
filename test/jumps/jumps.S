.text
.global _start
_start:

    mov     $0,%rax
    mov     $0,%rdx
    cmp     %rax,%rdx
    jz jump0
    int3
jump0:
    jnz fail

    mov     $5,%rax
    mov     $0,%rdx
    cmp     %rax,%rdx
    jnz jump1
    int3
jump1:
    jz fail

    mov     $5,%rax
    mov     $4,%rdx
    cmp     %rax,%rdx
    js jump2
    int3
jump2:
    jns fail

    mov     $4,%rax
    mov     $5,%rdx
    cmp     %rax,%rdx
    jns jump3
    int3
jump3:
    js fail

    mov     $4611686018427387904,%rax
    mov     $2,%rdx
    imul    %rax,%rdx
    jo jump4
    int3
jump4:
    jno fail

    mov     $4611686018427387903,%rax
    mov     $2,%rdx
    imul    %rax,%rdx
    jno jump5
    int3
jump5:
    jo fail

    mov     $5,%rax
    mov     $4,%rdx
    cmp     %rax,%rdx
    jc jump6
    int3
jump6:
    jnc fail

    mov     $4,%rax
    mov     $5,%rdx
    cmp     %rax,%rdx
    jnc jump7
    int3
jump7:
    jc fail

    mov     $5,%rax
    mov     $5,%rdx
    cmp     %rax,%rdx
    jnc jump8
    int3
jump8:
    jc fail

    mov     $5,%rax
    mov     $4,%rdx
    cmp     %rax,%rdx
    jl jump9
    int3
jump9:
    jge fail

    mov     $4,%rax
    mov     $5,%rdx
    cmp     %rax,%rdx
    jge jump10
    int3
jump10:
    jl fail

    mov     $5,%rax
    mov     $5,%rdx
    cmp     %rax,%rdx
    jge jump11
    int3
jump11:
    jl fail

    mov     $-4,%rax
    mov     $-5,%rdx
    cmp     %rax,%rdx
    jl jump12
    int3
jump12:
    jge fail

    mov     $-5,%rax
    mov     $-4,%rdx
    cmp     %rax,%rdx
    jge jump13
    int3
jump13:
    jl fail

    mov     $-5,%rax
    mov     $-6,%rdx
    cmp     %rax,%rdx
    jge jump14
    int3
jump14:
    jl fail

    mov     $5,%rax
    mov     $4,%rdx
    cmp     %rax,%rdx
    jbe jump15
    int3
jump15:
    ja fail

    mov     $5,%rax
    mov     $5,%rdx
    cmp     %rax,%rdx
    jbe jump16
    int3
jump16:
    ja fail

    mov     $4,%rax
    mov     $5,%rdx
    cmp     %rax,%rdx
    ja jump17
    int3
jump17:
    jbe fail

    mov     $5,%rax
    mov     $4,%rdx
    cmp     %rax,%rdx
    jle jump18
    int3
jump18:
    jg fail

    mov     $5,%rax
    mov     $5,%rdx
    cmp     %rax,%rdx
    jle jump19
    int3
jump19:
    jg fail

    mov     $4,%rax
    mov     $5,%rdx
    cmp     %rax,%rdx
    jg jump20
    int3
jump20:
    jle fail

    mov     $-4,%rax
    mov     $-5,%rdx
    cmp     %rax,%rdx
    jle jump21
    int3
jump21:
    jg fail

    mov     $-5,%rax
    mov     $-5,%rdx
    cmp     %rax,%rdx
    jle jump22
    int3
jump22:
    jg fail

    mov     $-5,%rax
    mov     $-4,%rdx
    cmp     %rax,%rdx
    jg jump23
    int3
jump23:
    jle fail

    mov     $0,%rax
    mov     $0,%rdx
    add     %rax,%rdx
    jp jump24
    int3
jump24:
    jnp fail

    mov     $0,%rax
    mov     $1,%rdx
    add     %rax,%rdx
    jnp jump25
    int3
jump25:
    jp fail

    mov     $0,%rax
    mov     $2,%rdx
    add     %rax,%rdx
    jnp jump26
    int3
jump26:
    jp fail

    mov     $0,%rax
    mov     $3,%rdx
    add     %rax,%rdx
    jp jump27
    int3
jump27:
    jnp fail

    mov     $0,%rax
    mov     $4,%rdx
    add     %rax,%rdx
    jnp jump28
    int3
jump28:
    jp fail

    mov     $0,%rax
    mov     $5,%rdx
    add     %rax,%rdx
    jp jump29
    int3
jump29:
    jnp fail

    mov     $0,%rax
    mov     $6,%rdx
    add     %rax,%rdx
    jp jump30
    int3
jump30:
    jnp fail

    mov     $0,%rax
    mov     $7,%rdx
    add     %rax,%rdx
    jnp jump31
    int3
jump31:
    jp fail

    mov     $0,%rax
    mov     $256,%rdx
    add     %rax,%rdx
    jp jump32
    int3
jump32:
    jnp fail

    mov     $0,%rax
    mov     $512,%rdx
    add     %rax,%rdx
    jp jump33
    int3
jump33:
    jnp fail

    mov     $0,%rax
    mov     $768,%rdx
    add     %rax,%rdx
    jp jump34
    int3
jump34:
    jnp fail

    mov     $0,%rbx
    mov     $1,%rax
    int     $0x80
fail:
    int3
