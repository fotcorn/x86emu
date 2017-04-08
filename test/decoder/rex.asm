.text
.global  _start
_start:


# 40 							E 			REX 											Access to new 8-bit registers

# 41 							E 			REX.B 											Extension of r/m field, base field, or opcode reg field
push %r8

# 42 							E 			REX.X 											Extension of SIB index field
# 43 							E 			REX.XB 											REX.X and REX.B combination

# 44 							E 			REX.R 											Extension of ModR/M reg field
#TODO

# 45 							E 			REX.RB 											REX.R and REX.B combination
#TODO

# 46 							E 			REX.RX 											REX.R and REX.X combination
# 47 							E 			REX.RXB 										REX.R, REX.X and REX.B combination

# 48 							E 			REX.W 											64 Bit Operand Size
mov %rax, %rax

# 49 							E 			REX.WB 											REX.W and REX.B combination
mov %rax, %r8

# 4A 							E 			REX.WX 											REX.W and REX.X combination
# 4B 							E 			REX.WXB 										REX.W, REX.X and REX.B combination

# 4C 							E 			REX.WR 											REX.W and REX.R combination
mov %r8, %rax

# 4D 							E 			REX.WRB 										REX.W, REX.R and REX.B combination
mov %r8, %r9

# 4E 							E 			REX.WRX 										REX.W, REX.R and REX.X combination
# 4F 							E 			REX.WRXB 										REX.W, REX.R, REX.X and REX.B combination
