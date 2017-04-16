.text
.global  _start
_start:
push %rax
push %rbx
push %rcx
push %rdx
push %rsp
push %rbp
push %rsi
push %rdi
mov %eax,%eax
mov %ebx,%ebx
mov %ecx,%ecx
mov %edx,%edx
mov %esp,%esp
mov %ebp,%ebp
mov %esi,%esi
mov %edi,%edi

int     $0x80
