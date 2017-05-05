.text
.global  _start
_start:

mov    $0xffffffffffffffff,%eax
movzx  %dil,%ebp
movzx  %dil,%edi
setz   %dil
movslq %ebp,%rdi
movslq %ebp,%r8

int     $0x80
