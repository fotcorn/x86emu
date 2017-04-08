.text
.global  _start
_start:

push    %r8
mov     %rax,%r8
movq    %rax,-0x4(%r8)
movq    -0x4(%r8),%rax

# TODO: 8B %rax,%r8, 8B %r8,%rax
