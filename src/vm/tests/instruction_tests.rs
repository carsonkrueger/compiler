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

    cpu.rg_at_mut(10).expect("").set_i32(256);
    let mut i = Instruction {
        opcode: Opcode::Str,
        op1: 10,
        op2: 1,
    };
    i.execute(&mut cpu);
    assert_eq!(cpu.memory.get_data_seg_i32(1).expect(""), 256);

    cpu.rg_at_mut(63).expect("").set_i32(200000000);
    let mut i = Instruction {
        opcode: Opcode::Str,
        op1: 63,
        op2: 2,
    };
    i.execute(&mut cpu);
    assert_eq!(cpu.memory.get_data_seg_i32(2).expect(""), 200000000);
}

#[test]
fn str2_test() {
    let path = String::from("HelloWorld.bin");
    let mut cpu = Cpu::new(&path);

    cpu.rg_at_mut(0).expect("").set_i32(-100);
    cpu.rg_at_mut(1).expect("").set_i32(0);
    let mut i = Instruction {
        opcode: Opcode::Str2,
        op1: 0,
        op2: 1,
    };
    i.execute(&mut cpu);
    assert_eq!(cpu.memory.get_data_seg_i32(0).expect(""), -100);

    cpu.rg_at_mut(10).expect("").set_i32(256);
    cpu.rg_at_mut(1).expect("").set_i32(1);
    let mut i = Instruction {
        opcode: Opcode::Str2,
        op1: 10,
        op2: 1,
    };
    i.execute(&mut cpu);
    assert_eq!(cpu.memory.get_data_seg_i32(1).expect(""), 256);

    cpu.rg_at_mut(63).expect("").set_i32(200000000);
    cpu.rg_at_mut(2).expect("").set_i32(10);
    let mut i = Instruction {
        opcode: Opcode::Str2,
        op1: 63,
        op2: 2,
    };
    i.execute(&mut cpu);
    assert_eq!(cpu.memory.get_data_seg_i32(10).expect(""), 200000000);
}

#[test]
fn ldr_test() {
    let path = String::from("HelloWorld.bin");
    let mut cpu = Cpu::new(&path);

    cpu.memory.set_i32(77, -77);
    let i = Instruction {
        opcode: Opcode::Ldr,
        op1: 0,
        op2: 77,
    };
    i.execute(&mut cpu);
    assert_eq!(cpu.rg_at_ref(0).expect("").get_i32(), -77);

    cpu.memory.set_i32(4, 6);
    let i = Instruction {
        opcode: Opcode::Ldr,
        op1: 0,
        op2: 4,
    };
    i.execute(&mut cpu);
    assert_eq!(cpu.rg_at_ref(0).expect("").get_i32(), 6);
}

#[test]
fn ldr2_test() {
    let path = String::from("HelloWorld.bin");
    let mut cpu = Cpu::new(&path);

    cpu.memory.set_i32(77, -77);
    cpu.rg_at_mut(63).expect("").set_i32(77);
    let i = Instruction {
        opcode: Opcode::Ldr2,
        op1: 0,
        op2: 63,
    };
    i.execute(&mut cpu);
    assert_eq!(cpu.rg_at_ref(0).expect("").get_i32(), -77);

    cpu.memory.set_i32(3, 256);
    cpu.rg_at_mut(12).expect("").set_i32(3);
    let i = Instruction {
        opcode: Opcode::Ldr2,
        op1: 15,
        op2: 12,
    };
    i.execute(&mut cpu);
    assert_eq!(cpu.rg_at_ref(15).expect("").get_i32(), 256);
}

#[test]
fn jmp_test() {
    let path = String::from("HelloWorld.bin");
    let mut cpu = Cpu::new(&path);

    let i = Instruction {
        opcode: Opcode::Jmp,
        op1: 25,
        op2: 69,
    };
    i.execute(&mut cpu);
    assert_eq!(cpu.get_pc(), 25);

    let i = Instruction {
        opcode: Opcode::Jmp,
        op1: -256,
        op2: 69,
    };
    i.execute(&mut cpu);
    assert_eq!(cpu.get_pc(), -256);
}

#[test]
fn jmr_test() {
    let path = String::from("HelloWorld.bin");
    let mut cpu = Cpu::new(&path);

    cpu.rg_at_mut(25).expect("").set_i32(-600);
    let i = Instruction {
        opcode: Opcode::Jmr,
        op1: 25,
        op2: 69,
    };
    i.execute(&mut cpu);
    assert_eq!(cpu.get_pc(), -600);
}

#[test]
fn bnz_test() {
    let path = String::from("HelloWorld.bin");
    let mut cpu = Cpu::new(&path);

    cpu.rg_at_mut(1).expect("").set_i32(-1);
    let i = Instruction {
        opcode: Opcode::Bnz,
        op1: 1,
        op2: 11,
    };
    i.execute(&mut cpu);
    assert_eq!(cpu.get_pc(), 11);

    cpu.rg_at_mut(1).expect("").set_i32(0);
    let i = Instruction {
        opcode: Opcode::Bnz,
        op1: 1,
        op2: 12,
    };
    i.execute(&mut cpu);
    assert_ne!(cpu.get_pc(), 12);

    cpu.rg_at_mut(1).expect("").set_i32(1);
    let i = Instruction {
        opcode: Opcode::Bnz,
        op1: 1,
        op2: 13,
    };
    i.execute(&mut cpu);
    assert_eq!(cpu.get_pc(), 13);
}

#[test]
fn bgt_test() {
    let path = String::from("HelloWorld.bin");
    let mut cpu = Cpu::new(&path);

    cpu.rg_at_mut(1).expect("").set_i32(-1);
    let i = Instruction {
        opcode: Opcode::Bgt,
        op1: 1,
        op2: 11,
    };
    i.execute(&mut cpu);
    assert_ne!(cpu.get_pc(), 11);

    cpu.rg_at_mut(1).expect("").set_i32(0);
    let i = Instruction {
        opcode: Opcode::Bgt,
        op1: 1,
        op2: 12,
    };
    i.execute(&mut cpu);
    assert_ne!(cpu.get_pc(), 12);

    cpu.rg_at_mut(1).expect("").set_i32(1);
    let i = Instruction {
        opcode: Opcode::Bgt,
        op1: 1,
        op2: 13,
    };
    i.execute(&mut cpu);
    assert_eq!(cpu.get_pc(), 13);
}

#[test]
fn blt_test() {
    let path = String::from("HelloWorld.bin");
    let mut cpu = Cpu::new(&path);

    cpu.rg_at_mut(1).expect("").set_i32(-1);
    let i = Instruction {
        opcode: Opcode::Blt,
        op1: 1,
        op2: 11,
    };
    i.execute(&mut cpu);
    assert_eq!(cpu.get_pc(), 11);

    cpu.rg_at_mut(1).expect("").set_i32(0);
    let i = Instruction {
        opcode: Opcode::Blt,
        op1: 1,
        op2: 12,
    };
    i.execute(&mut cpu);
    assert_ne!(cpu.get_pc(), 12);

    cpu.rg_at_mut(1).expect("").set_i32(1);
    let i = Instruction {
        opcode: Opcode::Blt,
        op1: 1,
        op2: 13,
    };
    i.execute(&mut cpu);
    assert_ne!(cpu.get_pc(), 13);
}

#[test]
fn brz_test() {
    let path = String::from("HelloWorld.bin");
    let mut cpu = Cpu::new(&path);

    cpu.rg_at_mut(1).expect("").set_i32(-1);
    let i = Instruction {
        opcode: Opcode::Brz,
        op1: 1,
        op2: 11,
    };
    i.execute(&mut cpu);
    assert_ne!(cpu.get_pc(), 11);

    cpu.rg_at_mut(1).expect("").set_i32(0);
    let i = Instruction {
        opcode: Opcode::Brz,
        op1: 1,
        op2: 12,
    };
    i.execute(&mut cpu);
    assert_eq!(cpu.get_pc(), 12);

    cpu.rg_at_mut(1).expect("").set_i32(1);
    let i = Instruction {
        opcode: Opcode::Brz,
        op1: 1,
        op2: 13,
    };
    i.execute(&mut cpu);
    assert_ne!(cpu.get_pc(), 13);
}
