print('.text')
print('.global  _start')
print('_start:')

registers64 = (
 'rax',
 'rbx',
 'rcx',
 'rdx',
 'rsp',
 'rbp',
 'rsi',
 'rdi',
 'r8',
 'r9',
 'r10',
 'r11',
 'r12',
 'r13',
 'r14',
 'r15',
)

registers32 = (
    'eax',
    'ebx',
    'ecx',
    'edx',
    'esp',
    'ebp',
    'esi',
    'edi',
)

for r1 in registers64:
    for r2 in registers64:
        print('mov %{},%{}'.format(r1, r2))
