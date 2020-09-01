#!/bin/bash
#riscv64-unknown-elf-gcc main.c -march=rv64id -o main3 -ffreestanding -nostdlib
riscv32-unknown-elf-gcc main.c -march=rv32id -o main -ffreestanding -nostdlib
