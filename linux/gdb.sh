#!/usr/bin/env bash
gdb-multiarch \
	-ex "target remote localhost:1234" \
	-ex "break *0x24762a3" \
	-ex "set step-mode on" \
	-ex "c" \
	-ex "disconnect" \
	-ex "set architecture i386:x86-64:intel" \
	-ex "target remote localhost:1234" \
	-ex "layout regs"

# 0x10026b
# 0x24735d0
# 0x24735ea
# 0x2476450
# 0x24764fa
# 0x24770ed
# 0x2476fe4
# 0x2476d20
# 0x2476d48
# 0x2476db6
# 0x2476dc4
# 0x2476df4
# 0x2476e0c
# 0x24770f5
# 0x2476da9
# 0x2476291

# 0x24762a3 => somewhere in __putstr