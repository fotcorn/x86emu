import re
with open('gdb.txt') as f:
    lines = f.readlines()

stos = False

for line in lines:
    if re.match('.* in ?? ()', line):
        continue

    match = re.match('=> 0x[a-z0-9]+:\t(.*)\n', line)
    if not match:
        b = False
        for x in ['cs', 'ss', 'ds', 'es', 'fs', 'gs', 'eflags']:
            if line.startswith(x):
                b = True
                break
        if b:
            continue

        match = re.match('([a-z0-9]+ +0x[a-f0-9]+).*', line)
        if match:
            line = match.group(1)

        print(line.strip())
        continue
    
    line = match.group(1)
    if line == 'rep stos %rax,%es:(%rdi)':
        if stos:
            continue
        else:
            stos = True

    print(line.strip())
