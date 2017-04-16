.text
.global  _start
_start:
mov %rbx, 0x1(%rax)
mov %rbx, %rax

int     $0x80
