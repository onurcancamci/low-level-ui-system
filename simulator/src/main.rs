use elfloader::ElfBinary;
use std::fs;

mod instruction;
mod machine;
mod memory;
mod processor;
mod syscall;
mod util;

use machine::Machine;
use memory::Memory;
use util::*;

fn main() {
    //let bytes = [1u8];
    //let new_b = [bytes[0], 0, 0, 0];
    //let x = 1u32;
    //let xb = from_u32(x);
    //println!("{}", to_u32(&new_b));
    //println!("{:?}", xb);

    let binary_blob = fs::read("../main").unwrap();
    let binary = ElfBinary::new("main", binary_blob.as_slice()).unwrap();
    let mem = Memory::new(&binary, &binary_blob);
    let mut machine = Machine::new(mem);
    machine.run();
}
