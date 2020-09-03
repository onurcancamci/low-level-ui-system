use elfloader::ElfBinary;

const SP: usize = 2;

#[derive(Debug)]
pub(crate) struct Memory {
    _start: usize,
    segments: Vec<MemorySegment>,
    registers: [u32; 32],
    pc: u32,
}

#[derive(Debug)]
pub(crate) struct MemorySegment {
    start: usize,
    size: usize,
    content: Vec<u8>,
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
            });
        }
        //stack
        segments.push(MemorySegment {
            start: 2143289328usize,
            size: 2usize.pow(22),
            content: vec![0u8; 2usize.pow(22)],
        });
        let mut mem = Memory {
            segments,
            _start: binary.entry_point() as usize,
            registers: [0u32; 32],
            pc: binary.entry_point() as u32,
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

    pub fn get_entry_point(&self) -> usize {
        self._start
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
}
