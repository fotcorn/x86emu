.text
.global  _start
_start:

mov    $0xffffffffffffffff,%eax
movzx  %dil,%ebp
movzx  %dil,%edi
setz   %dil
/*movslq %ebp,%rdi
cmp    $0x2c,0x10(%rsp,%rdi,1)
*/
int     $0x80
