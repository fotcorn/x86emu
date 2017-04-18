.text
.global  _start
_start:

movabs $0x3fffffffffff,%rax

int     $0x80
