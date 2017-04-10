.text
.global  _start
_start:

movq    %rax,-0x4(%edi)
movq    %rax,-0x4(%rdi)
movq    %rax,-0x4(%r8)

movq    $5,-0x4(%edi)
movq    $5,-0x4(%rdi)
movq    $5,-0x4(%r8)

andb   $0xfd,0x211(%rdi)
cmpb   $0x0,0x1ef(%rdi)

