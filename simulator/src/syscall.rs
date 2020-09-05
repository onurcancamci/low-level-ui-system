use crate::memory::Memory;

pub(crate) struct Syscall;

impl Syscall {
    pub fn call(mem: &mut Memory, code: i32, args: [i32; 7]) -> i32 {
        match code {
            500 => {
                //exit
                //println!("EXIT");
                std::process::exit(args[0]);
            }
            501 => {
                //print_int
                println!("{}", args[0]);
                0
            }
            502 => {
                //put char
                //println!("{:?}", args);
                print!("{}", args[0] as u8 as char);
                1
            }
            503 => {
                //malloc
                mem.malloc(args[0] as usize, 0) as i32
            }
            504 => {
                //free
                mem.free(args[0] as u32) as i32
            }
            _ => -1,
        }
    }
}
