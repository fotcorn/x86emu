#!/usr/bin/env bash
diet gcc test-i386.c -o test
cargo run -- --loader elf test  --symbol _start
