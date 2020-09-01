entry0:
start:
addi sp, sp, -32
sw ra, 28(sp)
sw s0, 24(sp)
addi s0, sp, 32
li a1, 2
li a0, 1
jal ra, foo
sw a0, -20(s0)
li a1, 3
lw a0, -20(s0)
jal ra, foo
li a1, 2
li a0, 1
jal ra, test
mv a5, a0
mv a0, a5
lw ra, 28(sp)
lw s0, 24(sp)
addi sp, sp, 32
ret
__internal_syscall:
addi sp, sp, -48
sw s0, 44(sp)
addi s0, sp, 48
sw a0, -20(s0)
sw a1, -24(s0)
sw a2, -28(s0)
sw a3, -32(s0)
sw a4, -36(s0)
sw a5, -40(s0)
sw a6, -44(s0)
lw a0, -24(s0)
lw a1, -28(s0)
lw a2, -32(s0)
lw a3, -36(s0)
lw a4, -40(s0)
lw a5, -44(s0)
lw a7, -20(s0)
ecall
mv a5, a0
mv a0, a5
lw s0, 44(sp)
addi sp, sp, 48
ret
foo:
addi sp, sp, -32
sw s0, 28(sp)
addi s0, sp, 32
sw a0, -20(s0)
sw a1, -24(s0)
lw a4, -20(s0)
lw a5, -24(s0)
add a5, a4, a5
mv a0, a5
lw s0, 28(sp)
addi sp, sp, 32
ret
test:
addi sp, sp, -32
sw ra, 28(sp)
sw s0, 24(sp)
addi s0, sp, 32
sw a0, -20(s0)
sw a1, -24(s0)
li a6, 0
li a5, 0
li a4, 0
li a3, 0
li a2, 0
li a1, 0
li a0, 1
jal ra, __internal_syscall
mv a5, a0
mv a0, a5
lw ra, 28(sp)
lw s0, 24(sp)
addi sp, sp, 32
ret

