extern early_serial_base
extern boot_params
extern lines,cols,vidport
extern vidmem
extern memmove
extern __putstr

global __puthex

__puthex:; Function begin
        push    r12                                     ; 2C90 _ 41: 54
        push    rbp                                     ; 2C92 _ 55
        mov     eax, 48                                 ; 2C93 _ B8, 00000030
        push    rbx                                     ; 2C98 _ 53
        mov     rbx, rdi                                ; 2C99 _ 48: 89. FB
        mov     r12d, 60                                ; 2C9C _ 41: BC, 0000003C
        shl     rbx, 40                                 ; 2CA2 _ 48: C1. E3, 28
        sub     rsp, 16                                 ; 2CA6 _ 48: 83. EC, 10
        mov     word [rsp+0EH], ax                      ; 2CAA _ 66: 89. 44 24, 0E
        lea     rbp, [rsp+0EH]                          ; 2CAF _ 48: 8D. 6C 24, 0E
        jmp     ?_289                                   ; 2CB4 _ EB, 23

; Filling space: 0AH
; Filler type: Multi-byte NOP
;       db 66H, 2EH, 0FH, 1FH, 84H, 00H, 00H, 00H
;       db 00H, 00H

ALIGN   16
?_287:  add     eax, 48                                 ; 2CC0 _ 83. C0, 30
        mov     byte [rsp+0EH], al                      ; 2CC3 _ 88. 44 24, 0E
?_288:  mov     rdi, rbp                                ; 2CC7 _ 48: 89. EF
        sub     r12d, 4                                 ; 2CCA _ 41: 83. EC, 04
        call    __putstr                                ; 2CCE _ E8, 00000000(rel)
        cmp     r12d, -4                                ; 2CD3 _ 41: 83. FC, FC
        jz      ?_290                                   ; 2CD7 _ 74, 1F
?_289:  mov     rax, rbx                                ; 2CD9 _ 48: 89. D8
        mov     ecx, r12d                               ; 2CDC _ 44: 89. E1
        shr     rax, cl                                 ; 2CDF _ 48: D3. E8
        and     eax, 0FH                                ; 2CE2 _ 83. E0, 0F
        cmp     rax, 9                                  ; 2CE5 _ 48: 83. F8, 09
        jbe     ?_287                                   ; 2CE9 _ 76, D5
        add     eax, 87                                 ; 2CEB _ 83. C0, 57
        mov     byte [rsp+0EH], al                      ; 2CEE _ 88. 44 24, 0E
        jmp     ?_288                                   ; 2CF2 _ EB, D3

; Filling space: 4H
; Filler type: Multi-byte NOP
;       db 0FH, 1FH, 40H, 00H

ALIGN   8
?_290:  add     rsp, 16                                 ; 2CF8 _ 48: 83. C4, 10
        pop     rbx                                     ; 2CFC _ 5B
        pop     rbp                                     ; 2CFD _ 5D
        pop     r12                                     ; 2CFE _ 41: 5C
        ret                                             ; 2D00 _ C3
; __puthex End of function

