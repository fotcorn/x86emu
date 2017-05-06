.text
.global  _start
_start:

movsbl  %dil,%ebp
movsbl  %dil,%edi
movslq  %ebp,%rdi
movslq  %ebp,%r8
movslq  (%rdx,%rax,4),%rax

int     $0x80
