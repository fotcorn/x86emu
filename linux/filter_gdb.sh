#!/usr/bin/env bash
python filter_gdb.py | \
sed -e 's/movs[bwl][wlq]/movsx /g' | \
sed -e 's/movz[bwl][wlq]/movzx /g' | \
sed -e 's/movabs/mov   /g' | \
sed -e 's/leaveq/leave/g' | \
sed -e 's/retq/ret/g' | \
sed -e 's/pushq/push /g' | \
sed -e 's/pushfq/pushf /g' | \
sed -e 's/popfq/popf /g' | \
sed -e 's/repz ret/ret/g' | \
sed -e 's/xchg   %ax,%ax/nop/g' | \
sed -e 's/jo.*/jo/g' | \
sed -e 's/jno.*/jno/g' | \
sed -e 's/jb.*/jb/g' | \
sed -e 's/jae.*/jae/g' | \
sed -e 's/je.*/je/g' | \
sed -e 's/jne.*/jne/g' | \
sed -e 's/jbe.*/jbe/g' | \
sed -e 's/ja.*/ja/g' | \
sed -e 's/js.*/js/g' | \
sed -e 's/jns.*/jns/g' | \
sed -e 's/jp.*/jp/g' | \
sed -e 's/jnp.*/jnp/g' | \
sed -e 's/jl.*/jl/g' | \
sed -e 's/jge.*/jge/g' | \
sed -e 's/jle.*/jle/g' | \
sed -e 's/jg.*/jg/g' | \
sed -e 's/call.*/call/g' | \
sed -e 's/jmp.*/jmp/g' | \
sed -e 's/nop.*/nop/g' | \
sed -e 's/0x0(/(/g' | \
sed -e '/^$/d' | \
sed -e 's/[ \t]*$//' | \
sed -e 's/\s*#.*$//' \
> gdb2.txt
