use crate::vm::{cpu::Cpu, instruction::Instruction, opcode::Opcode};

#[test]
fn execute_test() {
    let path = String::from("HelloWorld.bin");
    let mut cpu = Cpu::new(&path);

    let mut i = Instruction {
        opcode: Opcode::Jmp,
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
        opcode: Opcode::Mov,
        op1: 5,
        op2: 63,
    };

    cpu.rg_at_mut(63).expect("Should not crash").set_i32(100);
    i.execute(&mut cpu);
    assert_eq!(cpu.rg_at_ref(5).expect("Should not crash").get_i32(), 100);

    cpu.rg_at_mut(63)
        .expect("Should not crash")
        .set_i32(1000000000);
    i.execute(&mut cpu);
    assert_eq!(
        cpu.rg_at_ref(5).expect("Should not crash").get_i32(),
        1000000000
    );

    cpu.rg_at_mut(63)
        .expect("Should not crash")
        .set_i32(-1000000000);
    i.execute(&mut cpu);
    assert_eq!(
        cpu.rg_at_ref(5).expect("Should not crash").get_i32(),
        -1000000000
    );
}

#[test]
fn movi_test() {
    let path = String::from("HelloWorld.bin");
    let mut cpu = Cpu::new(&path);

    let mut i = Instruction {
        opcode: Opcode::Movi,
        op1: 0,
        op2: 63,
    };

    cpu.rg_at_mut(0).expect("Should not crash").set_i32(100);
    i.execute(&mut cpu);
    assert_eq!(cpu.rg_at_ref(0).expect("").get_i32(), 63);

    i.op2 = 256000000;

    i.execute(&mut cpu);
    assert_eq!(cpu.rg_at_ref(0).expect("").get_i32(), 256000000);

    i.op2 = -1000000000;

    i.execute(&mut cpu);
    assert_eq!(
        cpu.rg_at_ref(0).expect("Should not crash").get_i32(),
        -1000000000
    );
}

#[test]
fn lda_test() {
    let path = String::from("HelloWorld.bin");
    let mut cpu = Cpu::new(&path);

    let mut i = Instruction {
        opcode: Opcode::Lda,
        op1: 0,
        op2: 6,
    };

    i.execute(&mut cpu);
    assert_eq!(cpu.rg_at_ref(0).expect("").get_i32(), 6);
}

#[test]
fn str_test() {
    let path = String::from("HelloWorld.bin");
    let mut cpu = Cpu::new(&path);

    cpu.rg_at_mut(0).expect("").set_i32(-100);
    let mut i = Instruction {
        opcode: Opcode::Str,
        op1: 0,
        op2: 0,
    };

    i.execute(&mut cpu);
    assert_eq!(cpu.memory.get_data_seg_i32(0).expect(""), -100);
}
