use elfloader::ElfBinary;
use std::sync::RwLock;

const SP: usize = 2;

#[derive(Debug)]
pub(crate) struct Memory {
    _start: usize,
    segments: Vec<MemorySegment>,
    registers: [u32; 32],
    pc: u32,
    pub debug: bool,
}

#[derive(Debug)]
pub(crate) struct MemorySegment {
    start: usize,
    size: usize,
    content: Vec<u8>,
    persistent: bool,
}

impl Memory {
    pub fn new(binary: &ElfBinary, blob: &[u8]) -> Self {
        let mut segments = vec![];
        for header_part in binary.program_headers() {
            let start = header_part.virtual_addr() as usize;
            let size = header_part.mem_size() as usize;
            let mut content = vec![0u8; size];

            let file_size = header_part.file_size() as usize;
            let file_offset = header_part.offset() as usize;
            content[0..file_size].clone_from_slice(&blob[file_offset..(file_offset + file_size)]);

            segments.push(MemorySegment {
                start,
                size,
                content,
                persistent: true,
            });
        }
        //stack
        segments.push(MemorySegment {
            start: 2143289328usize,
            size: 2usize.pow(22),
            content: vec![0u8; 2usize.pow(22)],
            persistent: true,
        });
        let mut mem = Memory {
            segments,
            _start: binary.entry_point() as usize,
            registers: [0u32; 32],
            pc: binary.entry_point() as u32,
            debug: false,
        };
        mem.registers[SP] = 2143289328 + 2u32.pow(22);
        mem
    }

    pub fn set_register(&mut self, val: u32, ind: u8) {
        if ind == 0 {
            return;
        }
        self.registers[ind as usize] = val;
    }

    pub fn get_register(&self, ind: u8) -> u32 {
        self.registers[ind as usize]
    }

    pub fn set_pc(&mut self, val: u32) {
        self.pc = val;
    }

    pub fn get_pc(&self) -> u32 {
        self.pc
    }

    pub fn incr_pc(&mut self) {
        self.pc += 4;
    }

    pub fn get_instr(&self) -> &[u8] {
        self.read(self.pc as usize, 4)
    }

    pub fn read(&self, start: usize, len: usize) -> &[u8] {
        let segment = self
            .segments
            .iter()
            .find(|s| start >= s.start && start < (s.start + s.size))
            .expect("Invalid Memory Segment");
        let content_start = start - segment.start;
        &segment.content[content_start..(content_start + len)]
    }

    pub fn read_mut(&mut self, start: usize, len: usize) -> &mut [u8] {
        let segment = self
            .segments
            .iter_mut()
            .find(|s| start >= s.start && start < (s.start + s.size))
            .expect("Invalid Memory Segment");
        let content_start = start - segment.start;
        &mut segment.content[content_start..(content_start + len)]
    }

    pub fn malloc(&mut self, size: usize, init: u8) -> u32 {
        let last = self.segments.last().unwrap();
        let mut last_end = last.start + last.size;
        match last_end % 4 {
            0 => {}
            1 => {
                last_end += 3;
            }
            2 => {
                last_end += 2;
            }
            3 => {
                last_end += 1;
            }
            _ => unreachable!(),
        };
        let seg = MemorySegment {
            start: last_end,
            size: size,
            content: vec![init; size],
            persistent: false,
        };
        self.segments.push(seg);
        last_end as u32
    }

    pub fn free(&mut self, start: u32) -> u8 {
        let seg = self
            .segments
            .iter()
            .enumerate()
            .find(|(i, s)| s.start == start as usize);
        let i = match seg {
            Some((
                i,
                MemorySegment {
                    persistent: false, ..
                },
            )) => i,
            _ => {
                return 1;
            }
        };
        self.segments.remove(i);
        0
    }
}
