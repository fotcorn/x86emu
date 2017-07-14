#!/usr/bin/env bash
mkdir -p tmp
diet gcc test-i386.c -o tmp/out
./tmp/out | cut -c 12- | tr "[:upper:]" "[:lower:]" > tmp/native.txt
cargo run --release -- --loader elf tmp/out  --symbol _start | cut -c 12- | tr "[:upper:]" "[:lower:]" > tmp/emulator.txt
diff -u tmp/emulator.txt tmp/native.txt | diff-so-fancy --color
