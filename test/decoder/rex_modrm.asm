.text
.global  _start
_start:

mov     %r8,%rax
movq    %r8,-0x4(%rax)
movq    -0x4(%rax),%r8

int     $0x80
