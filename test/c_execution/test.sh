#!/usr/bin/env bash
mkdir -p temp/

gcc $1 -o temp/out
gcc -c $1 -o temp/out.o
./temp/out
cargo run -- --loader elf --cpu emu temp/out --symbol main
