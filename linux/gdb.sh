#!/usr/bin/env bash
gdb-multiarch \
	-ex "target remote localhost:1234" \
	-ex "break *0x10026b" \
	-ex "set step-mode on" \
	-ex "c" \
	-ex "disconnect" \
	-ex "set architecture i386:x86-64:intel" \
	-ex "target remote localhost:1234" \
	-ex "layout regs"

# 0x10026b
# 0x24735d0
# 0x24735ea
# 0x2476450
# 0x24764fa
# 0x24770ed
# 0x2476fe4
# 0x2476d20
# 0x2476d48
# 0x2476db6
# 0x2476dc4
# 0x2476df4
# 0x2476e0c
# 0x24770f5
# 0x2476da9
# 0x2476291

# 0x24762a3 => somewhere in __putstr


# desktop
# 0x2481df0  extract_kernel
# 0x2481FBE  extract_lernel line 403, before __decompress
# 0x247fed0  zlib_inflateInit2
# 0x247ff30  zlib_inflate
# 0x2480115  zlib_inflate after movzx  -0x1(%r12,%rsp,1),%eax
