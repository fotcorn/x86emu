.text
.global  _start
_start:

mov    %rax,0x0(%rip)
mov    0x0(%rip),%rax
movq   $0xb0000,0x0(%rip)
movl   $0x3b4,0x0(%rip)

int     $0x80
