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
    let binary_blob = fs::read("../main").unwrap();
    let binary = ElfBinary::new("main", binary_blob.as_slice()).unwrap();
    //binary
    //    .for_each_symbol(|e| println!("{}", e.name()))
    //    .unwrap();
    // TODO: Init GP register with value in the symbol table
    let mem = Memory::new(&binary, &binary_blob);
    let mut machine = Machine::new(mem);
    machine.run();
}
