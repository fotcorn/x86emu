#!/usr/bin/env bash
mkdir -p temp
python jumps.py > jumps.S
as jumps.S -o temp/jumps.o
ld -s -o temp/jumps temp/jumps.o
./temp/jumps
