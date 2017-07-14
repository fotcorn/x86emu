#!/usr/bin/env bash
gdb test \
	-ex "break _start" \
	-ex "set step-mode on" \
	-ex "set height 0" \
    -ex "r"

exit

while 1
x/i $pc
stepi
info registers
end
