#[cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Debug)]
pub(crate) enum Instruction {
    LUI { imm: i32, rd: u8 },            // Load Upper Immediate
    AUIPC { imm: i32, rd: u8 },          // LUI for PC and stores Result in the 'rd'
    JAL { imm: i32, rd: u8 }, // Jump relative to current instruction address. Stores next address in 'rd'. (PC+4)(Not jump location)
    JALR { imm: i32, rd: u8, rs1: u8 }, // Same as JAL but jump to 'rs1' + 'imm'
    BEQ { imm: i32, rs1: u8, rs2: u8 }, // if rs1 == rs2 : jump
    BNE { imm: i32, rs1: u8, rs2: u8 }, // if rs1 != rs2 : jump
    BLT { imm: i32, rs1: u8, rs2: u8 }, // if (i32)rs1 < (i32)rs2 : jump
    BGE { imm: i32, rs1: u8, rs2: u8 }, // if (i32)rs1 >= (i32)rs2 : jump
    BLTU { imm: i32, rs1: u8, rs2: u8 }, // if (u32)rs1 < (u32)rs2 : jump
    BGEU { imm: i32, rs1: u8, rs2: u8 }, // if (u32)rs1 >= (u32)rs2 : jump
    LB { imm: i32, rs1: u8, rd: u8 }, // // LH but 8 bit
    LH { imm: i32, rs1: u8, rd: u8 }, // LW but reads 16 bit and sign extends
    LW { imm: i32, rs1: u8, rd: u8 }, // M[rs1 + imm] -> rd
    LBU { imm: i32, rs1: u8, rd: u8 }, // LB but zero extends
    LHU { imm: i32, rs1: u8, rd: u8 }, // LH but zero extends
    SB { imm: i32, rs1: u8, rs2: u8 }, // SH but 8 bit
    SH { imm: i32, rs1: u8, rs2: u8 }, // Stores lower 16 bit of register
    SW { imm: i32, rs1: u8, rs2: u8 }, // rs2 -> M[rs1 + imm]
    ADDI { imm: i32, rs1: u8, rd: u8 }, // rs1 + (i32)imm -> rd // overflow vs ignored
    SLTI { imm: i32, rs1: u8, rd: u8 }, // if (i32)rs1 < (i32)imm : 1 -> rd   else: 0 -> rd
    SLTIU { imm: i32, rs1: u8, rd: u8 }, // if (u32)rs1 < (u32)((i32)imm) : 1 -> rd   else 0 -> rd
    // ^  if SLTIU rd, rs1, 1 => if rs1 == 0 : 1 -> rd   else:  0 -> rd
    XORI { imm: i32, rs1: u8, rd: u8 }, // rs1 ^ imm -> rd
    ORI { imm: i32, rs1: u8, rd: u8 }, // rs1 | imm -> rd
    ANDI { imm: i32, rs1: u8, rd: u8 }, // rs1 & imm -> rd
    SLLI { shift: u8, rs1: u8, rd: u8 }, // rs1 << shift -> rd (logical left shift)
    SRLI { shift: u8, rs1: u8, rd: u8 }, // rs1 >> shift -> rd (logical right shift)
    SRAI { shift: u8, rs1: u8, rd: u8 }, // rs1 >> shift -> rd (arithmetic right shift) (sign extends)
    ADD { rs1: u8, rs2: u8, rd: u8 }, // rs1 + rs2 -> rd // i32 arithmetic
    SUB { rs1: u8, rs2: u8, rd: u8 }, // rs2 - rs1 -> rd // i32
    SLL { rs1: u8, rs2: u8, rd: u8 }, // rs1 << rs2[4:0] -> rd
    SLT { rs1: u8, rs2: u8, rd: u8 }, // Like SLTI, rs1 < rs2
    SLTU { rs1: u8, rs2: u8, rd: u8 }, // Like SLTIU
    // ^ if SLTU rd, x0, rs2 => if rs2 != 0 : 1 -> rd else: 0 -> rd
    XOR { rs1: u8, rs2: u8, rd: u8 }, // rs1 ^ rs2 -> rd
    SRL { rs1: u8, rs2: u8, rd: u8 }, // rs1 >> rs2[4:0] -> rd
    SRA { rs1: u8, rs2: u8, rd: u8 }, // rs1 >> rs2[4:0] -> rd (arithmetic)
    OR { rs1: u8, rs2: u8, rd: u8 }, // rs1 | rs2 -> rd
    AND { rs1: u8, rs2: u8, rd: u8 }, // rs1 & rs2 -> rd
    //FENCE { rs1: u8, rd: u8, fm: u8, pred: u8, succ: u8 },
    ECALL,
    EBREAK,
}

const MASK_OP: u32 = 0b1111111;
const MASK_LUI_IMM: u32 = 0b11111111111111111111000000000000;
const MASK_RD: u32 = 0b111110000000;
const MASK_11_0: u32 = 0b111111111111 << 20;
const MASK_11_0_EXTEND: u32 = MASK_LUI_IMM;

impl Instruction {
    pub fn new(inst: u32) -> Self {
        let op = inst & MASK_OP;
        //println!("{:032b}", (u32::MAX & (0b1111111111 << 21)) >> 20);
        //println!("{:032b}", (u32::MAX & (0b0 << 21) | (1 << 20)) >> 9);
        //println!("{:032b}", (1u32 << 31) >> (31 - 12));
        //println!("{:032b}", (u32::MAX & (0b1111 << 8)) >> 0);
        //println!("{:032b}", inst);
        //let bytes = inst.to_le_bytes();
        //println!(
        //    "0x{:02X}{:02X}{:02X}{:02X}",
        //    bytes[0], bytes[1], bytes[2], bytes[3]
        //);
        match op {
            0b0110111 => {
                // LUI
                let imm = inst & MASK_LUI_IMM;
                let rd = get_rd(inst);
                Instruction::LUI {
                    rd,
                    imm: imm as i32,
                }
            }
            0b0010111 => {
                // AUIPC
                let imm = inst & MASK_LUI_IMM;
                let rd = get_rd(inst);
                Instruction::AUIPC {
                    rd,
                    imm: imm as i32,
                }
            }
            0b1101111 => {
                // JAL
                let mut base = 0;
                let b20 = (inst & (1 << 31)) >> 11;
                let b10_1 = (inst & (0b1111111111 << 21)) >> 20;
                let b11 = (inst & (1 << 20)) >> 9;
                let b19_12 = inst & (0b11111111 << 12);
                if (inst & (1 << 31)) != 0 {
                    let ext = 0b111111111111 << 20;
                    base = base | ext;
                }
                base = base | b20 | b19_12 | b11 | b10_1;

                let rd = get_rd(inst);
                Instruction::JAL {
                    rd,
                    imm: base as i32,
                }
            }
            0b1100111 => {
                // JALR
                let imm = get_imm_11_0(inst) as i32;
                let rs1 = get_rs1(inst);
                let rd = get_rd(inst);
                Instruction::JALR { imm, rs1, rd }
            }
            0b1100011 => {
                // BEQ, BNE, BLT, BGE, BLTU, BGEU
                let func3 = get_func3(inst);
                let imm = get_b_imm(inst) as i32;
                let rs2 = get_rs2(inst);
                let rs1 = get_rs1(inst);

                match func3 {
                    0b000 => Instruction::BEQ { imm, rs1, rs2 },
                    0b001 => Instruction::BNE { imm, rs1, rs2 },
                    0b100 => Instruction::BLT { imm, rs1, rs2 },
                    0b101 => Instruction::BGE { imm, rs1, rs2 },
                    0b110 => Instruction::BLTU { imm, rs1, rs2 },
                    0b111 => Instruction::BGEU { imm, rs1, rs2 },
                    _ => {
                        panic!("Undefined B instuction");
                    }
                }
            }
            0b0000011 => {
                // LB, LH, LW, LBU, LHU
                let imm = get_imm_11_0(inst) as i32;
                let rs1 = get_rs1(inst);
                let rd = get_rd(inst);
                let func3 = get_func3(inst);

                match func3 {
                    0b000 => Instruction::LB { imm, rs1, rd },
                    0b001 => Instruction::LH { imm, rs1, rd },
                    0b010 => Instruction::LW { imm, rs1, rd },
                    0b100 => Instruction::LBU { imm, rs1, rd },
                    0b101 => Instruction::LHU { imm, rs1, rd },
                    _ => {
                        panic!("Undefined L instuction");
                    }
                }
            }
            0b0100011 => {
                // SB, SH, SW
                let imm = get_s_imm(inst) as i32;
                let rs1 = get_rs1(inst);
                let rs2 = get_rs2(inst);
                let func3 = get_func3(inst);

                match func3 {
                    0b000 => Instruction::SB { imm, rs1, rs2 },
                    0b001 => Instruction::SH { imm, rs1, rs2 },
                    0b010 => Instruction::SW { imm, rs1, rs2 },
                    _ => {
                        panic!("Undefined S instuction");
                    }
                }
            }
            0b0010011 => {
                // ADDI, SLTI, SLTIU, XORI, ORI, ANDI, SLLI, SRLI, SRAI
                let func3 = get_func3(inst);
                let func7 = get_func7(inst);
                let rs2 = get_rs2(inst);
                let imm = get_imm_11_0(inst) as i32;
                let rs1 = get_rs1(inst);
                let rd = get_rd(inst);

                match func3 {
                    0b000 => Instruction::ADDI { imm, rd, rs1 },
                    0b010 => Instruction::SLTI { imm, rd, rs1 },
                    0b011 => Instruction::SLTIU { imm, rd, rs1 },
                    0b100 => Instruction::XORI { imm, rd, rs1 },
                    0b110 => Instruction::ORI { imm, rd, rs1 },
                    0b111 => Instruction::ANDI { imm, rd, rs1 },
                    0b001 => Instruction::SLLI {
                        shift: rs2,
                        rs1,
                        rd,
                    },
                    0b101 => {
                        if func7 == 0 {
                            Instruction::SRLI {
                                shift: rs2,
                                rs1,
                                rd,
                            }
                        } else {
                            Instruction::SRAI {
                                shift: rs2,
                                rs1,
                                rd,
                            }
                        }
                    }
                    _ => {
                        panic!("Undefined I instuction");
                    }
                }
            }
            0b0110011 => {
                // ADD, SUB, SLL, SLT, SLTU, XOR, SRL, SRA, OR, AND
                let func7 = get_func7(inst);
                let func3 = get_func3(inst);
                let rs1 = get_rs1(inst);
                let rs2 = get_rs2(inst);
                let rd = get_rd(inst);

                match func3 {
                    0b000 if func7 == 0 => Instruction::ADD { rs2, rd, rs1 },
                    0b000 if func7 != 0 => Instruction::SUB { rs2, rd, rs1 },
                    0b001 => Instruction::SLL { rs2, rd, rs1 },
                    0b010 => Instruction::SLT { rs2, rd, rs1 },
                    0b011 => Instruction::SLTU { rs2, rd, rs1 },
                    0b100 => Instruction::XOR { rs2, rd, rs1 },
                    0b101 if func7 == 0 => Instruction::SRL { rs2, rd, rs1 },
                    0b101 if func7 != 0 => Instruction::SRA { rs2, rd, rs1 },
                    0b110 => Instruction::OR { rs2, rd, rs1 },
                    0b111 => Instruction::AND { rs2, rd, rs1 },
                    _ => {
                        panic!("Undefined R instuction");
                    }
                }
            }
            0b0001111 => {
                // FENCE
                unimplemented!()
            }
            0b1110011 => {
                // ECALL, EBREAK
                let imm = get_imm_11_0(inst);
                if imm == 0 {
                    Instruction::ECALL
                } else {
                    Instruction::EBREAK
                }
            }
            _ => {
                panic!("Undefined OP Code");
            }
        }
    }
}

fn get_rd(inst: u32) -> u8 {
    let masked = inst & (0b11111 << 7);
    (masked >> 7) as u8
}

fn get_rs1(inst: u32) -> u8 {
    let masked = inst & (0b11111 << 15);
    (masked >> 15) as u8
}

fn get_rs2(inst: u32) -> u8 {
    let masked = inst & (0b11111 << 20);
    (masked >> 20) as u8
}

fn get_func3(inst: u32) -> u8 {
    let masked = inst & (0b111 << 12);
    (masked >> 12) as u8
}

fn get_func7(inst: u32) -> u8 {
    ((inst & (0b1111111 << 25)) >> 20) as u8
}

fn get_b_imm(inst: u32) -> u32 {
    let mut base = 0;
    let b12 = (inst & (1u32 << 31)) >> (31 - 12);
    let b10_5 = (inst & (0b111111 << 25)) >> 20;
    let b11 = (inst & (1u32 << 7)) << 4;
    let b4_1 = (inst & (0b1111 << 8)) >> 7;
    if (inst & (1 << 31)) != 0 {
        let ext = 0b11111111111111111111 << 12;
        base = base | ext;
    }
    base = base | b12 | b4_1 | b11 | b10_5;
    base
}

fn get_s_imm(inst: u32) -> u32 {
    let mut base = 0;
    let b11_5 = (inst & (0b1111111 << 25)) >> 20;
    let b4_0 = (inst & (0b11111 << 7)) >> 7;
    if (inst & (1 << 31)) != 0 {
        let ext = 0b11111111111111111111 << 12;
        base = base | ext;
    }
    base = base | b11_5 | b4_0;
    base
}

fn get_imm_11_0(inst: u32) -> u32 {
    let mut base = (inst & MASK_11_0) >> 20;
    if (inst & (1 << 31)) != 0 {
        let ext = MASK_11_0_EXTEND;
        base = base | ext;
    }
    base
}
