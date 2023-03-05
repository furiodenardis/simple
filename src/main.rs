
pub mod instructions;

use instructions::*;

#[cfg(test)]
#[path = "./cpu_test.rs"]
mod cpu_test;

fn main() {
    let mut my_cpu = Cpu {
        reg: init_registers(),
        memory: [0; 65535],
        pc: 0
    };
    my_cpu.memory[0] = 0x00;
    my_cpu.memory[1] = 0x01;
    my_cpu.memory[2] = 0xde;
    my_cpu.memory[3] = 0xad;
    my_cpu.memory[4] = 0x01;
    my_cpu.memory[5] = 0x00;
    my_cpu.memory[6] = 0x00;
    my_cpu.memory[7] = 0x01;
    my_cpu.memory[8] = 0x02;
    my_cpu.memory[9] = 0x02;
    my_cpu.memory[10] = 0x00;
    my_cpu.memory[11] = 0x02;
    my_cpu.print_status();
    my_cpu.print_display();
    let next_op = my_cpu.fetch();
    my_cpu.decode(next_op);
    my_cpu.print_status();
    let next_op = my_cpu.fetch();
    my_cpu.decode(next_op);
    my_cpu.print_status();
    let next_op = my_cpu.fetch();
    my_cpu.decode(next_op);
    my_cpu.print_status();
}

struct Registers {
    register: [u16; 16],
    flag_zero: bool,
    flag_gt: bool,
    flag_lt: bool,
}

fn init_registers() -> Registers {
    Registers { 
        register: [0; 16], 
        flag_zero: false, 
        flag_gt: false, 
        flag_lt: false 
    }
}

struct Cpu {
    reg: Registers,
    memory: [u8; 0xFFFF],
    pc: usize
}

impl Cpu {

    fn print_status(&self) {
        println!("PC: [0X{:04X}] FLAGZ: [{}] FLAG_LT [{}] FLAG_GT [{}]", self.pc, self.reg.flag_zero, self.reg.flag_lt, self.reg.flag_gt);
        for (idx, val) in self.reg.register.into_iter().enumerate() {
            print!("REG[{:#02}]: [0X{:04X}] ", idx, val);
            if (idx + 1) % 8 == 0 {
                println!();
            }
        }
    }

    fn print_display(&self) {
        let start_of_display_mem: usize =  0xFFBF;
        let len_of_display_mem: usize =  0x0030;
    
        for i in 0..len_of_display_mem {
            let c = self.memory[start_of_display_mem + i];
            if c < 32 {
                print!(" ");
            } else {
                print!("{}", c as char);
            }
            if (i + 1) % 16 == 0 {
                println!();
            }
        }
    }

    fn print_op(&self, op : (u8, u8, u16)) {
        println!("op : {:?}", op);
    }

    // Our cpu has 32-bit instrutions with 8bit opcode 8bit mode and 16bit data (big-endian) 
    // TODO: checkout the byteorder crate
    fn fetch(&self) -> (u8, u8, u16) {
        let op = self.memory[self.pc];
        let mode = self.memory[self.pc + 1];
        let data = (self.memory[self.pc + 2] as u16) << 8 | (self.memory[self.pc + 3] as u16);
        (op, mode, data)
    }

    fn decode(&mut self, instruction: (u8, u8, u16)) {
        self.print_op(instruction);
        let (op, mode, data) = instruction;
        match op {
            0x00 =>  { load_direct!(self, mode, data); },
            0x01 =>  { load_register!(self, mode, data); },
            0x02 =>  { load_indirect!(self, mode, data); },
            _ => println!("NOT IMPLEMENTED")
        };
    }
}