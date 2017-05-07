#!/usr/bin/env bash
mkdir -p tmp/
gcc $1 -o tmp/out
gcc -c $1 -o tmp/out.o
cargo run --features print_instructions -- --loader elf --cpu emu tmp/out --symbol main --benchmark
