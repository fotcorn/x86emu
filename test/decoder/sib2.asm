.text
.global  _start
_start:

lea    0x1,%ecx
lea    0x1(,1),%ecx  # same as the line above

lea    0x1(%rax),%ecx
lea    0x1(%rax,1),%ecx  # same as the line above
lea    0x1(,%rax),%ecx
lea    0x1(,%rax,1),%ecx # same as the line above

lea    0x1(%rbp,%rbx,1),%ecx
lea    0xFFF(%rbp,%rbx,1),%ecx

int     $0x80
