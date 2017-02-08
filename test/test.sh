#!/usr/bin/env bash
mkdir -p temp/
python generate.py > temp/out.asm
as temp/out.asm -o temp/out.o
ld temp/out.o -o temp/out
objdump -d temp/out | tail -n +8 | cut -d$'\t' -f3 > temp/dis_objdump.asm
cargo run temp/out _start > temp/dis_emu.asm
diff -u temp/dis_objdump.asm temp/dis_emu.asm
