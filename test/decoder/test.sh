#!/usr/bin/env bash
mkdir -p tmp/
cat $1 > tmp/out.asm
echo "nop" >> tmp/out.asm
as tmp/out.asm -o tmp/out.o
ld tmp/out.o -o tmp/out
objdump -d tmp/out | tail -n +8 | cut -d$'\t' -f3 | head -n -1 | \
sed -e 's/movs[bwl][wlq]/movsx /g' | \
sed -e 's/movz[bwl][wlq]/movzx /g' | \
sed -e 's/movabs/mov   /g' | \
sed -e 's/callq/call /' | \
sed -e 's/leaveq/leave/g' | \
sed -e 's/call.*/call/g' | \
sed -e 's/retq/ret/g' | \
sed -e 's/\s*#.*$//' | \
sed -e 's/0x0(/(/g' | \
sed -e '/^$/d' \
> tmp/dis_objdump.asm

cargo run -- --loader elf --cpu print --symbol _start tmp/out | \
sed -e 's/call.*/call/g' | \
sed -e 's/0x0(/(/g' \
> tmp/dis_emu.asm

diff -u tmp/dis_objdump.asm tmp/dis_emu.asm
