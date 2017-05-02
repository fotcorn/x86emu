import re
with open('gdb.txt') as f:
    lines = f.readlines()

for line in lines:
    match = re.match('=> 0x[a-z0-9]+:\t(.*)\n', line)
    if not match:
        continue
    
    line = match.group(1)
    if line == 'rep stos %rax,%es:(%rdi)':
        continue

    print(line.strip())
