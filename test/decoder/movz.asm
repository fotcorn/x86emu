.text
.global  _start
_start:

movzbw  %ah,%ax
movzbl  %ah,%eax
movzbq  %r8b,%rax

movzwl  %ax,%eax
movzwq  %r8w,%rax


int     $0x80
