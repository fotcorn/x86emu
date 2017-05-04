#!/usr/bin/env bash
# howto run: cd linux; ./gdb_debug.sh > gdb.txt
# copy paste the last 5 lines (without #), after that run filter_gdb.sh

gdb-multiarch \
	-ex "target remote localhost:1234" \
	-ex "break *0x10026b" \
	-ex "set step-mode on" \
	-ex "c" \
	-ex "disconnect" \
	-ex "set architecture i386:x86-64:intel" \
	-ex "target remote localhost:1234" \
	-ex "set height 0"

# while 1
# x/i $pc
# stepi
# info registers
# end
