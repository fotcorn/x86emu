#!/usr/bin/env bash
python filter_gdb.py | \
sed -e 's/mov[lqbw]/mov /g' | \
sed -e 's/movs[bwl][wlq]/movsx /g' | \
sed -e 's/movz[bwl][wlq]/movzx /g' | \
sed -e 's/movabs/mov   /g' | \
sed -e 's/andb/and /g' | \
sed -e 's/cmpb/cmp /g' | \
sed -e 's/addl/add /g' | \
sed -e 's/leaveq/leave/g' | \
sed -e 's/retq/ret/g' | \
sed -e 's/je.*/jz/g' | \
sed -e 's/jae.*/jnc/g' | \
sed -e 's/ja.*/ja/g' | \
sed -e 's/jne.*/jnz/g' | \
sed -e 's/jbe.*/jbe/g' | \
sed -e 's/jle.*/jle/g' | \
sed -e 's/jg.*/jg/g' | \
sed -e 's/call.*/call/g' | \
sed -e 's/jmp.*/jmp/g' | \
sed -e 's/0x0(/(/g' | \
sed -e '/^$/d' | \
sed -e 's/[ \t]*$//' | \
sed -e 's/\s*#.*$//' \
> gdb2.txt
