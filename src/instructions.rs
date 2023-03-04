macro_rules! read_16 {
    ($cpu:ident,$address:ident) => {
        ($cpu.memory[$address as usize] as u16) << 8 | ($cpu.memory[($address + 1) as usize] as u16)
    }
}

macro_rules! load_direct {
    ($cpu:ident,$reg:ident,$data:ident) => {
        if $reg <= 7 {
            $cpu.reg.register[$reg as usize] = $data;
            $cpu.pc += 4;
        }
    
    }
}

macro_rules! load_register {
    ($cpu:ident,$reg:ident,$data:ident) => {
        if $reg <= 7 && $data <= 7 {
            $cpu.reg.register[$reg as usize] = $cpu.reg.register[$data as usize];
            $cpu.pc += 4;
        }
    }
}


macro_rules! load_indirect {
    ($cpu:ident,$reg:ident,$data:ident) => {
        if $reg <= 7 {
            $cpu.reg.register[$reg as usize] = read_16!($cpu, $data);
            $cpu.pc += 4;
        }
    }
}

pub(super) use read_16;

pub(super) use load_direct;    
pub(super) use load_register;
pub(super) use load_indirect;
