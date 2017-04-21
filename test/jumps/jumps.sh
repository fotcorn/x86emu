#!/usr/bin/env bash
mkdir -p tmp
python test/jumps/jumps.py > tmp/jumps.S
as tmp/jumps.S -o tmp/jumps.o
ld -o tmp/jumps tmp/jumps.o
./tmp/jumps
cargo run -- --loader elf --cpu emu tmp/jumps  --symbol _start > /dev/null
