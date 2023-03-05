#[cfg(test)]

#[test]
fn test_load_direct() {

    let mut my_cpu = super::Cpu {
        reg: super::init_registers(),
        memory: [0; 65535],
        pc: 0
    };    

    my_cpu.memory[0] = 0x00;
    my_cpu.memory[1] = 0x01;
    my_cpu.memory[2] = 0xde;
    my_cpu.memory[3] = 0xad;

    let next_op = my_cpu.fetch();
    my_cpu.decode(next_op);

    assert_eq!(my_cpu.reg.register[1], 0xdead);

}

#[test]
fn test_load_register() {

    let mut my_cpu = super::Cpu {
        reg: super::init_registers(),
        memory: [0; 65535],
        pc: 0
    };    

    my_cpu.memory[0] = 0x01;
    my_cpu.memory[1] = 0x00;
    my_cpu.memory[2] = 0x00;
    my_cpu.memory[3] = 0x01;
    my_cpu.reg.register[1] = 0xdead;

    let next_op = my_cpu.fetch();
    my_cpu.decode(next_op);

    assert_eq!(my_cpu.reg.register[0], 0xdead);

}

#[test]
fn test_load_indirect() {

    let mut my_cpu = super::Cpu {
        reg: super::init_registers(),
        memory: [0; 65535],
        pc: 0
    };    

    my_cpu.memory[0] = 0x02;
    my_cpu.memory[1] = 0x02;
    my_cpu.memory[2] = 0x00;
    my_cpu.memory[3] = 0x04;
    my_cpu.memory[4] = 0xde;
    my_cpu.memory[5] = 0xad;

    my_cpu.reg.register[1] = 0xdead;

    let next_op = my_cpu.fetch();
    my_cpu.decode(next_op);

    assert_eq!(my_cpu.reg.register[2], 0xdead);

}