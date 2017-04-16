.text
.global  _start
_start:

mov    -0x4(%ebp),%eax
mov    -0x4(%ebp),%rax
mov    -0x4(%rbp),%eax
mov    -0x4(%rbp),%rax

mov    %eax,-0x4(%ebp)
mov    %rax,-0x4(%ebp)
mov    %eax,-0x4(%rbp)
mov    %rax,-0x4(%rbp)

cmovs  %edx,%eax
cmovs  %eax,%edx

int     $0x80
