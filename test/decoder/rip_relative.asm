.text
.global  _start
_start:

mov    %rax,0xFF(%rip)
mov    0xFF(%rip),%rax
movq   $0xb0000,0xFF(%rip)
movl   $0x3b4,0xFF(%rip)

int     $0x80
