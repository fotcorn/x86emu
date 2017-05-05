#!/usr/bin/env bash
#objconv -fnasm ../../linux/linux/arch/x86/boot/compressed/misc.o
#mv ../../linux/linux/arch/x86/boot/compressed/misc.asm misc.asm
mkdir -p tmp
gcc -c main.c -o tmp/main.o
nasm -f elf64 puthex.asm -o tmp/misc.o
gcc tmp/main.o tmp/misc.o -o tmp/out
./tmp/out
cargo run -- --loader elf --cpu emu tmp/out  --symbol main > /dev/null
