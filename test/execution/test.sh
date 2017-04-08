#!/usr/bin/env bash
mkdir -p temp/
as $1 -o temp/out.o
ld -o temp/out temp/out.o
./temp/out
cargo run -- --loader elf --cpu emu temp/out  --symbol _start
