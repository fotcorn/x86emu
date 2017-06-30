#!/usr/bin/env bash
# howto run: cd linux; ./gdb_debug.sh > gdb.txt
# copy paste the last 6 lines, after that run filter_gdb.sh

gdb-multiarch \
	-ex "target remote localhost:1234" \
	-ex "break *0x10026b" \
	-ex "break *0xffffffff81000150" \
	-ex "set step-mode on" \
	-ex "c" \
	-ex "disconnect" \
	-ex "set architecture i386:x86-64:intel" \
	-ex "target remote localhost:1234" \
	-ex "set height 0"

exit

c
while 1
x/i $pc
stepi
info registers
end
