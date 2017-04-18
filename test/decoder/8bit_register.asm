.text
.global  _start
_start:
test %dil,%dil
int     $0x80
