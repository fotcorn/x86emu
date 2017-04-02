.text
.global  _start
_start:

# 40
mov   $0x40,%sil

# 41 							E 			REX.B 											Extension of r/m field, base field, or opcode reg field
push %r8

# 42 							E 			REX.X 											Extension of SIB index field
lea    -4(%rbx, %r8, 4), %eax

# 43 							E 			REX.XB 											REX.X and REX.B combination
lea    -4(%r8, %r9, 4), %eax

# 44 							E 			REX.R 											Extension of ModR/M reg field
mov %r9d, %eax

# 45 							E 			REX.RB 											REX.R and REX.B combination
mov %r9d, %r8d

# 46 							E 			REX.RX 											REX.R and REX.X combination
mov    -4(%rax, %r8, 4),%r8d

# 47 							E 			REX.RXB 										REX.R, REX.X and REX.B combination
lea    -4(%r8, %r9, 4), %r8d

# 48 							E 			REX.W 											64 Bit Operand Size
mov %rax, %rax

# 49 							E 			REX.WB 											REX.W and REX.B combination
mov %rax, %r8

# 4A 							E 			REX.WX 											REX.W and REX.X combination
lea    -4(%rbx, %r8, 4), %rax

# 4B 							E 			REX.WXB 										REX.W, REX.X and REX.B combination
lea    -4(%r8, %r9, 4), %rax

# 4C 							E 			REX.WR 											REX.W and REX.R combination
mov %r8, %rax

# 4D 							E 			REX.WRB 										REX.W, REX.R and REX.B combination
mov %r8, %r9

# 4E 							E 			REX.WRX 										REX.W, REX.R and REX.X combination
lea    -4(%rax, %r9, 4), %r10

# 4F 							E 			REX.WRXB 										REX.W, REX.R, REX.X and REX.B combination
lea    -4(%r8, %r9, 4), %r10
