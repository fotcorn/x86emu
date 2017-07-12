#!/usr/bin/env bash
mkdir -p tmp/

diet gcc $1 -static -o tmp/out
./tmp/out
cargo run -- --loader elf tmp/out --symbol _start
