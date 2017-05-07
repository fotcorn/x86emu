#!/usr/bin/env bash
cat emu.txt |
grep -v WARNING | \
sed -e '0,/lea    0x521f(%rip),%rdx/d' | \
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
sed -e 's/nop.*/nop/g' | \
sed -e 's/sar    $0x1,%eax/sar    %eax/g' | \
sed -e 's/shr    $0x1,%rdx/shr    %rdx/g' | \
sed -e 's/shr    $0x1,%eax/shr    %eax/g' | \
sed -e 's/VIDEO.*//g' | \
sed -e 's/0x0(/(/g' | \
sed -e '/^$/d' | \
sed -e 's/[ \t]*$//' | \
sed -e 's/mov    %eax,%rax/cltq/g' | \
sed -e 's/jmp.*/jmp/g' \
> emu2.txt
