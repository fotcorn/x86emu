.text
.global  _start
_start:

movsbl  %dil,%ebp
movsbl  %dil,%edi
movslq  %r8d,%rdi
movslq  %eax,%r8
movslq  %ebp,%rdi
movslq  %ebp,%r8
movslq  (%rdx,%rax,4),%rax

int     $0x80
