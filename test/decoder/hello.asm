.text
.global  _start
_start:
push   %rbp
mov    %rsp,%rbp
sub    $0x10,%rsp
movl   $0x5,-0x4(%rbp)
addl   $0xf,-0x4(%rbp)
mov    -0x4(%rbp),%eax
mov    %eax,%esi
mov    $0x400604,%edi
mov    $0x0,%eax
callq  0x400400
mov    -0x4(%rbp),%eax
lea    0x3(%rax),%edx
test   %eax,%eax
cmovs  %edx,%eax
sar    $0x2,%eax
mov    %eax,-0x4(%rbp)
mov    -0x4(%rbp),%eax
mov    %eax,%esi
mov    $0x400604,%edi
mov    $0x0,%eax
callq  0x400400
mov    $0x0,%eax
leaveq
retq
