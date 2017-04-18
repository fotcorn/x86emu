.text
.global  _start
_start:

test   $0x1fffff,%ebp
not %eax
neg %eax
mul %eax
imul %eax
div %eax
idiv %eax


int     $0x80
