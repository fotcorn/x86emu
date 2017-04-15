#!/usr/bin/env bash
mkdir -p temp
python jumps.py > jumps.S
as jumps.S -o temp/jumps.o
ld -o temp/jumps temp/jumps.o
./temp/jumps
cargo run -- --loader elf --cpu emu temp/jumps  --symbol _start
