fn main() {
    let mut my_cpu = Cpu {
        reg: init_registers(),
        memory: [65; 65535],
        pc: 0
    };
    my_cpu.memory[0] = 0x00;
    my_cpu.memory[1] = 0x01;
    my_cpu.memory[2] = 0xff;
    my_cpu.memory[3] = 0x00;
    my_cpu.print_status();
    my_cpu.print_display();
    let next_op = my_cpu.fetch();
    my_cpu.print_op(next_op);
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
        let START_OF_DISPLAY_MEM: usize =  0xFFBF;
        let LEN_OF_DISPLAY_MEM: usize =  0x0030;
    
        for i in (0..LEN_OF_DISPLAY_MEM) {
            let c = self.memory[START_OF_DISPLAY_MEM + i];
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

    fn fetch(&self) -> (u8, u8, u16) {
        let op = self.memory[self.pc];
        let mode = self.memory[self.pc + 1];
        let data = (self.memory[self.pc + 2] as u16) << 8 | (self.memory[self.pc + 3] as u16);
        (op, mode, data)
    }
}