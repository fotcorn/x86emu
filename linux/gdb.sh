#!/usr/bin/env bash
gdb-multiarch \
	-ex "target remote localhost:1234" \
	-ex "break *0x100200" \
	-ex "set step-mode on" \
	-ex "c" \
	-ex "disconnect" \
	-ex "set architecture i386:x86-64:intel" \
	-ex "target remote localhost:1234"
