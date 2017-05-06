.text
.global  _start
_start:

sete    %dil
sete    %al
sete    %r8b

int     $0x80
