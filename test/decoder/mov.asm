.text
.global  _start
_start:


mov $5, %ah
mov $5, %al
mov $5, %ax
mov $5, %eax
mov $5, %rax

int     $0x80
