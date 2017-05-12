.text
.global  _start
_start:

// 69           IMUL     r16/32/64   r/m16/32/64     imm16/32
imul    $260, %rdx, %rax
imul    $260, %eax, %edx
imul    $260, %ax, %dx

// 6B           IMUL     r16/32/64   r/m16/32/64     imm8
imul    $5, %rdx, %rax
imul    $5, %edx, %eax
imul    $5, %edx
imul    $5, %eax
imul    $5, %dx, %ax

/*
// F6 opcode 5  IMUL 	AX 	AL 	r/m8
imul %bh

// F7 opcode 5  IMUL 	rDX 	rAX 	r/m16/32/64
imul %bx
imul %eax
imul %rax
*/

// 0F AF        IMUL 	r16/32/64 	r/m16/32/64
imul    %rax, %rbx
imul    %eax, %ebx
imul    %ax, %bx

imul    0x1(%rbx), %rax
imul    0x1(%rbx), %eax
imul    0x1(%rbx), %ax

imul    0x1(%ebx), %rax
imul    0x1(%ebx), %eax
imul    0x1(%ebx), %ax

int     $0x80
