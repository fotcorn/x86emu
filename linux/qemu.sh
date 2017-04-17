#!/usr/bin/env bash
qemu-system-x86_64 -kernel linux/arch/x86/boot/bzImage -append "debug earlyprintk=vga" -S -s -no-kvm
