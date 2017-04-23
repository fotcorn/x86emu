#!/usr/bin/env python
import os
import sys
from glob import glob


for f in glob('./test/decoder/*.asm'):
    command = './test/decoder/test.sh {}'.format(f)
    print(command)
    if os.system(command) != 0:
        sys.exit(1)

for f in glob('./test/execution/*.S'):
    command = './test/execution/test.sh {}'.format(f)
    print(command)
    if os.system(command) != 0:
        sys.exit(1)

for f in glob('./test/c_execution/*.c'):
    command = './test/c_execution/test.sh {}'.format(f)
    print(command)
    if os.system(command) != 0:
        sys.exit(1)

command = './test/jumps/jumps.sh'
print(command)
if os.system(command) != 0:
    sys.exit(1)
