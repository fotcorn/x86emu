# x86emu
The beginning of a x86_64 emulator written in Rust

## Current features
* Implemented a big chunk of the x86_64 instruction set
* Can load a linux kernel and let it uncompress itself and set up page tables
* Can load and run some basic userland elf files

## Next steps
* Implement timers and interrupts
* Implement emulated hardware (PCI, Keyboard, Screen, virtio block and net devices etc.)
