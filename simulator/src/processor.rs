pub(crate) struct Processor;

use crate::instruction::Instruction;
use crate::memory::Memory;
use crate::util::*;

impl Processor {
    pub fn tick(mem: &mut Memory) {
        use Instruction::*;
        let inst = Processor::fetch(mem);
        println!("{:#?}", inst);
        match inst {
            LUI { imm, rd } => {
                mem.set_register(imm as u32, rd);
                mem.incr_pc();
            }
            AUIPC { imm, rd } => {
                mem.set_register((mem.get_pc() as i32 + imm) as u32, rd);
                mem.incr_pc();
            }
            JAL { imm, rd } => {
                let pc = mem.get_pc();
                mem.set_register(pc + 4, rd);
                mem.set_pc((pc as i32 + imm) as u32);
            }
            JALR { imm, rs1, rd } => {
                let rv = mem.get_register(rs1) as i32;
                mem.set_register(mem.get_pc() + 4, rd);
                mem.set_pc((rv + imm) as u32);
            }
            BEQ { imm, rs1, rs2 } => {
                if mem.get_register(rs1) == mem.get_register(rs2) {
                    mem.set_pc((mem.get_pc() as i32 + imm) as u32);
                } else {
                    mem.incr_pc();
                }
            }
            BNE { imm, rs1, rs2 } => {
                if mem.get_register(rs1) != mem.get_register(rs2) {
                    mem.set_pc((mem.get_pc() as i32 + imm) as u32);
                } else {
                    mem.incr_pc();
                }
            }
            BLT { imm, rs1, rs2 } => {
                if (mem.get_register(rs1) as i32) < (mem.get_register(rs2) as i32) {
                    mem.set_pc((mem.get_pc() as i32 + imm) as u32);
                } else {
                    mem.incr_pc();
                }
            }
            BGE { imm, rs1, rs2 } => {
                if (mem.get_register(rs1) as i32) >= (mem.get_register(rs2) as i32) {
                    mem.set_pc((mem.get_pc() as i32 + imm) as u32);
                } else {
                    mem.incr_pc();
                }
            }
            BLTU { imm, rs1, rs2 } => {
                if (mem.get_register(rs1) as u32) < (mem.get_register(rs2) as u32) {
                    mem.set_pc((mem.get_pc() as i32 + imm) as u32);
                } else {
                    mem.incr_pc();
                }
            }
            BGEU { imm, rs1, rs2 } => {
                if (mem.get_register(rs1) as u32) >= (mem.get_register(rs2) as u32) {
                    mem.set_pc((mem.get_pc() as i32 + imm) as u32);
                } else {
                    mem.incr_pc();
                }
            }
            LB { imm, rs1, rd } => {
                let bytes = mem.read((mem.get_register(rs1) as i32 + imm) as usize, 1);
                let sign = bytes[0] & (1 << 7);
                let ext = if sign == 0 { 0u8 } else { 0xffu8 };
                let new_bytes = [bytes[0], ext, ext, ext];
                let val = to_u32(&new_bytes);
                mem.set_register(val, rd);
                mem.incr_pc();
            }
            LH { imm, rs1, rd } => {
                let bytes = mem.read((mem.get_register(rs1) as i32 + imm) as usize, 2);
                let sign = bytes[1] & (1 << 7);
                let ext = if sign == 0 { 0u8 } else { 0xffu8 };
                let new_bytes = [bytes[0], bytes[1], ext, ext];
                let val = to_u32(&new_bytes);
                mem.set_register(val, rd);
                mem.incr_pc();
            }
            LW { imm, rs1, rd } => {
                let bytes = mem.read((mem.get_register(rs1) as i32 + imm) as usize, 4);
                let val = to_u32(&bytes);
                mem.set_register(val, rd);
                mem.incr_pc();
            }
            LBU { imm, rs1, rd } => {
                let bytes = mem.read((mem.get_register(rs1) as i32 + imm) as usize, 1);
                let new_bytes = [0, 0, 0, bytes[0]];
                let val = to_u32(&new_bytes);
                mem.set_register(val, rd);
                mem.incr_pc();
            }
            LHU { imm, rs1, rd } => {
                let bytes = mem.read((mem.get_register(rs1) as i32 + imm) as usize, 2);
                let new_bytes = [0, 0, bytes[0], bytes[1]];
                let val = to_u32(&new_bytes);
                mem.set_register(val, rd);
                mem.incr_pc();
            }
            SB { imm, rs1, rs2 } => {
                let reg_bytes = from_u32(mem.get_register(rs2));
                let bytes = mem.read_mut((mem.get_register(rs1) as i32 + imm) as usize, 1);
                bytes[0] = reg_bytes[0];
                mem.incr_pc();
            }
            SH { imm, rs1, rs2 } => {
                let reg_bytes = from_u32(mem.get_register(rs2));
                let bytes = mem.read_mut((mem.get_register(rs1) as i32 + imm) as usize, 2);
                bytes[0] = reg_bytes[0];
                bytes[1] = reg_bytes[1];
                mem.incr_pc();
            }
            SW { imm, rs1, rs2 } => {
                let reg_bytes = from_u32(mem.get_register(rs2));
                let bytes = mem.read_mut((mem.get_register(rs1) as i32 + imm) as usize, 4);
                bytes[0] = reg_bytes[0];
                bytes[1] = reg_bytes[1];
                bytes[2] = reg_bytes[2];
                bytes[3] = reg_bytes[3];
                mem.incr_pc();
            }
            ADDI { imm, rs1, rd } => {
                mem.set_register((mem.get_register(rs1) as i32 + imm) as u32, rd);
                mem.incr_pc();
            }
            SLTI { imm, rs1, rd } => {
                if (mem.get_register(rs1) as i32) < imm {
                    mem.set_register(1, rd);
                } else {
                    mem.set_register(0, rd);
                }
                mem.incr_pc();
            }
            SLTIU { imm, rs1, rd } => {
                if imm == 1 {
                    if rs1 == 0 {
                        mem.set_register(1, rd);
                    } else {
                        mem.set_register(0, rd);
                    }
                } else {
                    if (mem.get_register(rs1) as u32) < (imm as u32) {
                        mem.set_register(1, rd);
                    } else {
                        mem.set_register(0, rd);
                    }
                }
                mem.incr_pc();
            }
            XORI { imm, rs1, rd } => {
                mem.set_register(mem.get_register(rs1) ^ imm as u32, rd);
                mem.incr_pc();
            }
            ORI { imm, rs1, rd } => {
                mem.set_register(mem.get_register(rs1) | imm as u32, rd);
                mem.incr_pc();
            }
            ANDI { imm, rs1, rd } => {
                mem.set_register(mem.get_register(rs1) & imm as u32, rd);
                mem.incr_pc();
            }
            SLLI { shift, rs1, rd } => {
                mem.set_register(mem.get_register(rs1) << shift, rd);
                mem.incr_pc();
            }
            SRLI { shift, rs1, rd } => {
                mem.set_register(mem.get_register(rs1) >> shift, rd);
                mem.incr_pc();
            }
            SRAI { shift, rs1, rd } => {
                mem.set_register((mem.get_register(rs1) as i32 >> shift) as u32, rd);
                mem.incr_pc();
            }
            ADD { rs1, rs2, rd } => {
                mem.set_register(
                    ((mem.get_register(rs1) as i32) + (mem.get_register(rs2) as i32)) as u32,
                    rd,
                );
                mem.incr_pc();
            }
            SUB { rs1, rs2, rd } => {
                mem.set_register(
                    ((mem.get_register(rs2) as i32) - (mem.get_register(rs1) as i32)) as u32,
                    rd,
                );
                mem.incr_pc();
            }
            SLL { rs1, rs2, rd } => {
                mem.set_register(
                    mem.get_register(rs2) << (mem.get_register(rs1) & 0b11111),
                    rd,
                );
                mem.incr_pc();
            }
            SLT { rs2, rs1, rd } => {
                if (mem.get_register(rs1) as i32) < (mem.get_register(rs2) as i32) {
                    mem.set_register(1, rd);
                } else {
                    mem.set_register(0, rd);
                }
                mem.incr_pc();
            }
            SLTU { rs2, rs1, rd } => {
                if rs1 == 0 {
                    if rs2 != 0 {
                        mem.set_register(1, rd);
                    } else {
                        mem.set_register(0, rd);
                    }
                } else {
                    if (mem.get_register(rs1) as u32) < (mem.get_register(rs2)) {
                        mem.set_register(1, rd);
                    } else {
                        mem.set_register(0, rd);
                    }
                }
                mem.incr_pc();
            }
            XOR { rs1, rs2, rd } => {
                mem.set_register(
                    ((mem.get_register(rs2) as i32) ^ (mem.get_register(rs1) as i32)) as u32,
                    rd,
                );
                mem.incr_pc();
            }
            SRL { rs1, rs2, rd } => {
                mem.set_register(
                    mem.get_register(rs2) >> (mem.get_register(rs1) & 0b11111),
                    rd,
                );
                mem.incr_pc();
            }
            SRA { rs1, rs2, rd } => {
                mem.set_register(
                    ((mem.get_register(rs2) as i32) >> (mem.get_register(rs1) & 0b11111)) as u32,
                    rd,
                );
                mem.incr_pc();
            }
            OR { rs1, rs2, rd } => {
                mem.set_register(
                    ((mem.get_register(rs2) as i32) | (mem.get_register(rs1) as i32)) as u32,
                    rd,
                );
                mem.incr_pc();
            }
            AND { rs1, rs2, rd } => {
                mem.set_register(
                    ((mem.get_register(rs2) as i32) & (mem.get_register(rs1) as i32)) as u32,
                    rd,
                );
                mem.incr_pc();
            }
            ECALL => {
                println!("ECALL RECEIVED");
                std::process::exit(0);
            }
            EBREAK => println!("EBREAK RECEIVED"),
        }
    }

    fn fetch(mem: &Memory) -> Instruction {
        let bytes = mem.get_instr();
        let inst = Instruction::new(to_u32(bytes));
        inst
    }
}
