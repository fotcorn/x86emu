.text
.global  _start
_start:


movsx %al,%bx
movsx %al,%ebx
movsx %al,%rbx

movsx %r8b,%bx
movsx %r8b,%ebx
movsx %r8b,%rbx

movsx %ah,%bx
movsx %ah,%ebx

movsx %ax,%ebx
movsx %ax,%rbx


int     $0x80
