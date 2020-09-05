use crate::memory::Memory;
use crate::processor::Processor;

pub(crate) struct Machine {
    mem: Memory,
}

impl Machine {
    pub fn new(mem: Memory) -> Self {
        Machine { mem }
    }

    pub fn run(&mut self) -> u32 {
        loop {
            Processor::tick(&mut self.mem);
        }
    }
}
