.text
.global  _start
_start:

add $5, %al

int     $0x80
