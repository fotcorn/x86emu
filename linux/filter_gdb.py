import re
with open('gdb.txt') as f:
    lines = f.readlines()

stos = False

i = -1
while True:
    i += 1
    if i == len(lines):
        break
    line = lines[i]
    if re.match('.* in ?? ()', line):
        continue
    match = re.match('=> 0x[a-zA-Z0-9<>_ \+]+:\t(.*)\n', line)
    if match:  # instruction
        line = match.group(1)
        if line == 'rep stos %rax,%es:(%rdi)':
            while True:
                i += 1
                line = lines[i]
                match = re.match('=> 0x[a-z0-9]+:\t(.*)\n', line)
                if match and match.group(1) != 'rep stos %rax,%es:(%rdi)':
                    print('rep stos %rax,%es:(%rdi)')
                    line = match.group(1)
                    break
        if line == 'rep stos %eax,%es:(%rdi)':
            while True:
                i += 1
                line = lines[i]
                match = re.match('=> 0x[a-z0-9]+:\t(.*)\n', line)
                if match and match.group(1) != 'rep stos %eax,%es:(%rdi)':
                    print('rep stos %eax,%es:(%rdi)')
                    line = match.group(1)
                    break
    else:  # register value
        b = False
        # skip some uninteresting registers
        for x in ['cs', 'ss', 'ds', 'es', 'fs', 'gs', 'eflags']:
            if line.startswith(x):
                b = True
                break
        if b:
            continue

        # only print hex value of register
        match = re.match('([a-z0-9]+ +0x[a-f0-9]+).*', line)
        if match:
            line = match.group(1)

    print(line.strip())
