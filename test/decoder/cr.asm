.text
.global  _start
_start:

mov     %rax, %cr0
mov     %rax, %cr2
mov     %rax, %cr3
mov     %rax, %cr4
mov     %rax, %cr8
mov     %r8, %cr4


int     $0x80
