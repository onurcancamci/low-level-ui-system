use elfloader::ElfBinary;
use std::fs;

mod instruction;
mod memory;
mod processor;
mod util;

use memory::Memory;
use processor::Processor;
use util::*;

fn main() {
    let binary_blob = fs::read("../main").unwrap();
    let binary = ElfBinary::new("main", binary_blob.as_slice()).unwrap();
    println!("{:#X}   {}", binary.entry_point(), binary.entry_point());
    println!("size: {}", binary_blob.len());
    let virtual_offset = binary.program_headers().next().unwrap().virtual_addr();
    println!("{:#X?}", virtual_offset);
    let start = binary.entry_point() as usize - virtual_offset as usize;
    let inst = &binary_blob[start..(start + 4)];
    println!("{:#X?}", inst);
    println!(
        "{:08b} {:08b} {:08b} {:08b}",
        inst[3], inst[2], inst[1], inst[0]
    );

    println!("{:08b} {:08b} {:08b} {:08b}", 0x00, 0xfa, 0xff, 0xff);
    println!(
        "{} {} {}",
        0xfa & (1 << 7),
        to_u32(&[0x00, 0xfa, 0xff, 0xff]) as i32,
        0xfa00u16 as i16
    );
    let bytes = from_u32(1);
    let val = to_u32(&bytes);
    println!(
        "{:08b} {:08b} {:08b} {:08b}",
        bytes[0], bytes[1], bytes[2], bytes[3]
    );
    println!("{}", val);

    let mut mem = Memory::new(&binary, &binary_blob);
    //println!("{:?}", mem);
    for k in 0..0 {
        let inst = mem.read(mem.get_entry_point() + k * 4, 4);
        //println!("{:#X?}", inst);

        let inst_u32 = util::to_u32(inst);
        let inst_enum = instruction::Instruction::new(inst_u32);

        println!("{:#?}", inst_enum);
    }
    println!("--------------------------");

    for k in 0..50 {
        Processor::tick(&mut mem);
    }
}
