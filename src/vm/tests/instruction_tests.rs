use crate::vm::{cpu::Cpu, instruction::Instruction, opcode::Opcode};

#[test]
fn execute_test() {
    let path = String::from("HelloWorld.bin");
    let mut cpu = Cpu::new(&path);

    let mut i = Instruction {
        opcode: Opcode::try_from(1).expect("Should not crash"),
        op1: 5,
        op2: 69,
    };
    i.execute(&mut cpu);
    assert_eq!(cpu.get_pc(), 5);

    i.op1 = 3;
    i.execute(&mut cpu);
    assert_eq!(cpu.get_pc(), 3);

    i.op1 = 11;
    i.execute(&mut cpu);
    assert_eq!(cpu.get_pc(), 11);

    i.op1 = 500;
    i.execute(&mut cpu);
    assert_eq!(cpu.get_pc(), 500);
}

#[test]
fn mov_test() {
    let path = String::from("HelloWorld.bin");
    let mut cpu = Cpu::new(&path);

    let mut i = Instruction {
        opcode: Opcode::try_from(7).expect("Should not crash"),
        op1: 5,
        op2: 63,
    };

    cpu.rg_at_mut(63).expect("Should not crash").set_i32(100);
    i.execute(&mut cpu);
    assert_eq!(cpu.rg_at_ref(5).expect("Should not crash").get_i32(), 100);
}

#[test]
fn movi_test() {
    let path = String::from("HelloWorld.bin");
    let mut cpu = Cpu::new(&path);
}
