#!/usr/bin/env bash
mkdir -p temp/
cat $1 > temp/out.asm
echo "nop" >> temp/out.asm
as temp/out.asm -o temp/out.o
ld temp/out.o -o temp/out
objdump -d temp/out | tail -n +8 | cut -d$'\t' -f3 | head -n -1 | \
sed -e 's/movl/mov /g' | \
sed -e 's/callq/call /' | \
sed -e 's/addl/add /g' | \
sed -e 's/leaveq/leave/g' | \
sed -e 's/call.*/call/g' | \
sed -e 's/retq/ret/g' | \
sed -e 's/cmovs/cmov /g' \
> temp/dis_objdump.asm
cargo run -- --loader elf --cpu print --symbol _start temp/out | sed -e 's/call.*/call/g' > temp/dis_emu.asm
diff -u temp/dis_objdump.asm temp/dis_emu.asm
