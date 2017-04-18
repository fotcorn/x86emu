.text
.global  _start
_start:

sar %cl,%eax

int     $0x80
