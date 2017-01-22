
        global  _start

        section .text
_start:


        mov     rax, 127
        mov     rax, 128
        mov     rax, 129
        mov     rax, 254
        mov     rax, 255
        mov     rax, 256


        ; write(1, message, 13)

        ;push rax
        ;push ax

        ;push R8L
        ;push R8W
        ;push R8D
        ;push R8

	
        ;mov     rax, 1                  ; system call 1 is write
        ;mov     rdi, 1                  ; file handle 1 is stdout
        ;mov     rsi, message            ; address of string to output
        ;mov     rdx, 13                 ; number of bytes
        ;syscall                         ; invoke operating system to do the write

        ; exit(0)
        mov     eax, 60                 ; system call 60 is exit
        xor     rdi, rdi                ; exit code 0
        syscall                         ; invoke operating system to exit
message:
        db      "Hello, World", 10      ; note the newline at the end

