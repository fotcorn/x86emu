#!/usr/bin/env bash
gdb-multiarch \
	-ex "target remote localhost:1234" \
	-ex "break *0x10026b" \
	-ex "set step-mode on" \
	-ex "c" \
	-ex "disconnect" \
	-ex "set architecture i386:x86-64:intel" \
	-ex "target remote localhost:1234" \
	-ex "set logging on" \
	-ex "set height 0"

while 1
x/i $pc
info registers
stepi
end
