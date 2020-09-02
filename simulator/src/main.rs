use elfloader::ElfBinary;
use std::fs;

mod instruction;
mod memory;
mod util;

use memory::Memory;

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

    let mut mem = Memory::new(&binary, &binary_blob);
    println!("{:?}", mem);

    let inst = mem.read(mem.get_entry_point(), 4);
    println!("{:#X?}", inst);

    let inst_u32 = util::to_u32(inst);
    let inst_enum = instruction::Instruction::new(inst_u32);

    println!("{:#?}", inst_enum);
}
