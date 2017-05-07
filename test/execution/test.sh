#!/usr/bin/env bash
mkdir -p tmp/
as $1 -o tmp/out.o
ld -o tmp/out tmp/out.o
./tmp/out
cargo run -- --loader elf tmp/out  --symbol _start
