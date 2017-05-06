.text
.global  _start
_start:

sete    %dil
sete    %al
sete    %r8b
sete   -0x39(%rax)
sete   -0x39(%rsp)

int     $0x80
