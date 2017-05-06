.text
.global  _start
_start:

mov    $0xffffffffffffffff,%eax
movzx  %dil,%ebp
movzx  %dil,%edi
setz   %dil
movslq %ebp,%rdi
movslq %ebp,%r8

movslq  (%rdx,%rax,4),%rax

int     $0x80
