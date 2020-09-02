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
    FENCE { rs1: u8, rd: u8, fm: u8, pred: u8, succ: u8 },
    ECALL {},
    EBREAK {},
}

const mask_op: u32 = 0b1111111;
const mask_lui_imm: u32 = 0b11111111111111111111000000000000;
const mask_rd: u32 = 0b111110000000;
const mask_sign: u32 = 1 << 31;
const mask_jarl_imm: u32 = 0b111111111111 << 20;

impl Instruction {
    pub fn new(inst: u32) -> Self {
        let op = inst & mask_op;
        //println!("{:032b}", (u32::MAX & (0b1111111111 << 21)) >> 20);
        //println!("{:032b}", (u32::MAX & (0b0 << 21) | (1 << 20)) >> 9);
        println!("{:032b}", 0b111111111111 << 19);
        match op {
            0b0110111 => {
                // LUI
                let imm = inst & mask_lui_imm;
                let rd = get_rd(inst);
                Instruction::LUI {
                    rd,
                    imm: imm as i32,
                }
            }
            0b0010111 => {
                // AUIPC
                let imm = inst & mask_lui_imm;
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
                unimplemented!()
            }
            0b1100011 => {
                // BEQ, BNE, BLT, BGE, BLTU, BGEU
                unimplemented!()
            }
            0b0000011 => {
                // LB, LH, LW, LBU, LHU
                unimplemented!()
            }
            0b0100011 => {
                // SB, SH, SW
                unimplemented!()
            }
            0b0010011 => {
                // ADDI, SLTI, SLTIU, XORI, ORI, ANDI, SLLI, SRLI, SRAI
                unimplemented!()
            }
            0b0110011 => {
                // ADD, SUB, SLL, SLT, SLTU, XOR, SRL, SRA, OR, AND
                unimplemented!()
            }
            0b0001111 => {
                // FENCE
                unimplemented!()
            }
            0b1110011 => {
                // ECALL, EBREAK
                unimplemented!()
            }
            _ => {
                panic!("Undefined OP Code");
            }
        }
    }
}

fn get_rd(inst: u32) -> u8 {
    let masked = inst & mask_rd;
    (masked >> 7) as u8
}
