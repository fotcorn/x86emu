#!/usr/bin/env bash
python filter_gdb.py | \
sed -e '0,/lea    0x521f(%rip),%rdx/d' | \
sed -e 's/movs[bwl][wlq]/movsx /g' | \
sed -e 's/movz[bwl][wlq]/movzx /g' | \
sed -e 's/movabs/mov   /g' | \
sed -e 's/leaveq/leave/g' | \
sed -e 's/retq/ret/g' | \
sed -e 's/repz ret/ret/g' | \
sed -e 's/xchg   %ax,%ax/nop/g' | \
sed -e 's/je.*/jz/g' | \
sed -e 's/jae.*/jnc/g' | \
sed -e 's/ja.*/ja/g' | \
sed -e 's/jne.*/jnz/g' | \
sed -e 's/jbe.*/jbe/g' | \
sed -e 's/jle.*/jle/g' | \
sed -e 's/jg.*/jg/g' | \
sed -e 's/js.*/js/g' | \
sed -e 's/call.*/call/g' | \
sed -e 's/jmp.*/jmp/g' | \
sed -e 's/nopl.*/nopl/g' | \
sed -e 's/0x0(/(/g' | \
sed -e '/^$/d' | \
sed -e 's/[ \t]*$//' | \
sed -e 's/\s*#.*$//' \
> gdb2.txt
