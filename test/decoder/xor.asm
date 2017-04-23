.text
.global  _start
_start:

xor $1234123, %rax
xor $1234, %eax
xor $1111, %ax

int     $0x80
