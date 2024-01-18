use crate::vm::{
    cpu::{Cpu, ExecuteResult},
    instruction::Instruction,
    opcode::Opcode,
};

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
fn cmp_test() {
    let path = String::from("HelloWorld.bin");
    let mut cpu = Cpu::new(&path);

    cpu.rg_at_mut(0).expect("").set_i32(0);
    cpu.rg_at_mut(1).expect("").set_i32(0);
    let i = Instruction {
        opcode: Opcode::Cmp,
        op1: 0,
        op2: 1,
    };
    i.execute(&mut cpu);
    assert_eq!(cpu.rg_at_ref(0).expect("").get_i32(), 0);
    assert_ne!(cpu.rg_at_ref(0).expect("").get_i32(), 1);
    assert_ne!(cpu.rg_at_ref(0).expect("").get_i32(), -1);

    cpu.rg_at_mut(0).expect("").set_i32(0);
    cpu.rg_at_mut(63).expect("").set_i32(100);
    let i = Instruction {
        opcode: Opcode::Cmp,
        op1: 0,
        op2: 63,
    };
    i.execute(&mut cpu);
    assert_eq!(cpu.rg_at_ref(0).expect("").get_i32(), -1);

    cpu.rg_at_mut(0).expect("").set_i32(-100);
    cpu.rg_at_mut(63).expect("").set_i32(-120);
    let i = Instruction {
        opcode: Opcode::Cmp,
        op1: 0,
        op2: 63,
    };
    i.execute(&mut cpu);
    assert_eq!(cpu.rg_at_ref(0).expect("").get_i32(), 1);

    cpu.rg_at_mut(0).expect("").set_i32(-256);
    cpu.rg_at_mut(63).expect("").set_i32(-256);
    let i = Instruction {
        opcode: Opcode::Cmp,
        op1: 0,
        op2: 63,
    };
    i.execute(&mut cpu);
    assert_eq!(cpu.rg_at_ref(0).expect("").get_i32(), 0);
    assert_ne!(cpu.rg_at_ref(0).expect("").get_i32(), 1);
    assert_ne!(cpu.rg_at_ref(0).expect("").get_i32(), -1);
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
fn ldb_test() {
    let mut cpu = Cpu::new(&String::from("HelloWorld.bin"));

    cpu.rg_at_mut(10).expect("").set_i32(5);
    let mut i = Instruction {
        opcode: Opcode::Ldb,
        op1: 10,
        op2: 100,
    };
    assert_eq!(i.execute(&mut cpu), ExecuteResult::Continue);
    assert_eq!(cpu.rg_at_ref(10).expect("").get_u8() as char, '\n');

    let mut i = Instruction {
        opcode: Opcode::Ldb,
        op1: 10,
        op2: 88,
    };
    i.execute(&mut cpu);
    assert_eq!(cpu.rg_at_ref(10).expect("").get_u8(), 5);
}

#[test]
fn ldb2_test() {
    let mut cpu = Cpu::new(&String::from("HelloWorld.bin"));

    cpu.rg_at_mut(10).expect("").set_i32(5);
    cpu.rg_at_mut(61).expect("").set_i32(100);
    let mut i = Instruction {
        opcode: Opcode::Ldb2,
        op1: 10,
        op2: 61,
    };
    assert_eq!(i.execute(&mut cpu), ExecuteResult::Continue);
    assert_eq!(cpu.rg_at_ref(10).expect("").get_u8() as char, '\n');

    cpu.rg_at_mut(22).expect("").set_i32(84);
    let mut i = Instruction {
        opcode: Opcode::Ldb2,
        op1: 10,
        op2: 22,
    };
    i.execute(&mut cpu);
    assert_eq!(cpu.rg_at_ref(10).expect("").get_u8(), 3);
}

#[test]
fn stb_test() {
    let mut cpu = Cpu::new(&String::from("HelloWorld.bin"));

    cpu.rg_at_mut(63).expect("").set_u8(255);
    let mut i = Instruction {
        opcode: Opcode::Stb,
        op1: 63,
        op2: 2,
    };
    i.execute(&mut cpu);
    assert_eq!(cpu.memory.get_any_u8(2).expect(""), 255);

    cpu.rg_at_mut(10).expect("").set_u8(16);
    let mut i = Instruction {
        opcode: Opcode::Stb,
        op1: 10,
        op2: 50,
    };
    i.execute(&mut cpu);
    assert_eq!(cpu.memory.get_any_u8(50).expect(""), 16);

    cpu.rg_at_mut(9).expect("").set_u8(151);
    let mut i = Instruction {
        opcode: Opcode::Stb,
        op1: 9,
        op2: 100,
    };
    i.execute(&mut cpu);
    assert_eq!(cpu.memory.get_any_u8(100).expect(""), 151);

    let mut i = Instruction {
        opcode: Opcode::Stb,
        op1: 10,
        op2: 101,
    };
    assert_eq!(
        i.execute(&mut cpu),
        ExecuteResult::Error(crate::vm::cpu::VMErr::MemoryErr(
            crate::vm::memory::MemoryErr::SetInsideCodeSegBounds(101)
        ))
    )
}

#[test]
fn stb2_test() {
    let mut cpu = Cpu::new(&String::from("HelloWorld.bin"));

    cpu.rg_at_mut(63).expect("").set_u8(255);
    cpu.rg_at_mut(2).expect("").set_u8(0);
    let mut i = Instruction {
        opcode: Opcode::Stb2,
        op1: 63,
        op2: 2,
    };
    i.execute(&mut cpu);
    assert_eq!(cpu.memory.get_any_u8(0).expect(""), 255);

    cpu.rg_at_mut(63).expect("").set_u8(161);
    cpu.rg_at_mut(10).expect("").set_u8(100);
    let mut i = Instruction {
        opcode: Opcode::Stb2,
        op1: 63,
        op2: 10,
    };
    i.execute(&mut cpu);
    assert_eq!(cpu.memory.get_any_u8(100).expect(""), 161);

    cpu.rg_at_mut(63).expect("").set_u8(161);
    cpu.rg_at_mut(10).expect("").set_u8(101);
    let mut i = Instruction {
        opcode: Opcode::Stb2,
        op1: 63,
        op2: 10,
    };
    assert_ne!(i.execute(&mut cpu), ExecuteResult::Continue);
    assert_ne!(i.execute(&mut cpu), ExecuteResult::Exit);
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

#[test]
fn sub_test() {
    let mut cpu = Cpu::new(&String::from("HelloWorld.bin"));

    cpu.rg_at_mut(0).expect("").set_i32(32);
    cpu.rg_at_mut(1).expect("").set_i32(32);
    let mut i = Instruction {
        opcode: Opcode::Sub,
        op1: 0,
        op2: 1,
    };
    i.execute(&mut cpu);
    assert_eq!(cpu.rg_at_ref(0).expect("").get_i32(), 0);

    cpu.rg_at_mut(10).expect("").set_i32(32);
    cpu.rg_at_mut(32).expect("").set_i32(-1000000032);
    let mut i = Instruction {
        opcode: Opcode::Sub,
        op1: 10,
        op2: 32,
    };
    i.execute(&mut cpu);
    assert_eq!(cpu.rg_at_ref(10).expect("").get_i32(), 1000000064);

    cpu.rg_at_mut(10).expect("").set_i32(32);
    cpu.rg_at_mut(32).expect("").set_i32(-1000000032);
    let mut i = Instruction {
        opcode: Opcode::Sub,
        op1: 32,
        op2: 10,
    };
    i.execute(&mut cpu);
    assert_eq!(cpu.rg_at_ref(32).expect("").get_i32(), -1000000064);
}

#[test]
fn add_test() {
    let mut cpu = Cpu::new(&String::from("HelloWorld.bin"));

    cpu.rg_at_mut(0).expect("").set_i32(32);
    cpu.rg_at_mut(1).expect("").set_i32(32);
    let mut i = Instruction {
        opcode: Opcode::Add,
        op1: 0,
        op2: 1,
    };
    i.execute(&mut cpu);
    assert_eq!(cpu.rg_at_ref(0).expect("").get_i32(), 64);

    cpu.rg_at_mut(10).expect("").set_i32(32);
    cpu.rg_at_mut(32).expect("").set_i32(-1000000032);
    let mut i = Instruction {
        opcode: Opcode::Add,
        op1: 10,
        op2: 32,
    };
    i.execute(&mut cpu);
    assert_eq!(cpu.rg_at_ref(10).expect("").get_i32(), -1000000000);
}

#[test]
fn adi_test() {
    let mut cpu = Cpu::new(&String::from("HelloWorld.bin"));

    cpu.rg_at_mut(0).expect("").set_i32(32);
    let mut i = Instruction {
        opcode: Opcode::Adi,
        op1: 0,
        op2: 1,
    };
    i.execute(&mut cpu);
    assert_eq!(cpu.rg_at_ref(0).expect("").get_i32(), 33);

    cpu.rg_at_mut(10).expect("").set_i32(32);
    let mut i = Instruction {
        opcode: Opcode::Adi,
        op1: 10,
        op2: -1032,
    };
    i.execute(&mut cpu);
    assert_eq!(cpu.rg_at_ref(10).expect("").get_i32(), -1000);
}

#[test]
fn div_test() {
    let mut cpu = Cpu::new(&String::from("HelloWorld.bin"));

    cpu.rg_at_mut(0).expect("").set_i32(32);
    cpu.rg_at_mut(1).expect("").set_i32(32);
    let mut i = Instruction {
        opcode: Opcode::Div,
        op1: 0,
        op2: 1,
    };
    i.execute(&mut cpu);
    assert_eq!(cpu.rg_at_ref(0).expect("").get_i32(), 1);

    cpu.rg_at_mut(10).expect("").set_i32(-1000);
    cpu.rg_at_mut(32).expect("").set_i32(32);
    let mut i = Instruction {
        opcode: Opcode::Div,
        op1: 10,
        op2: 32,
    };
    i.execute(&mut cpu);
    assert_eq!(cpu.rg_at_ref(10).expect("").get_i32(), -31);
}

#[test]
fn divi_test() {
    let mut cpu = Cpu::new(&String::from("HelloWorld.bin"));

    cpu.rg_at_mut(0).expect("").set_i32(32);
    let mut i = Instruction {
        opcode: Opcode::Divi,
        op1: 0,
        op2: 2,
    };
    i.execute(&mut cpu);
    assert_eq!(cpu.rg_at_ref(0).expect("").get_i32(), 16);

    cpu.rg_at_mut(10).expect("").set_i32(-1000);
    let mut i = Instruction {
        opcode: Opcode::Divi,
        op1: 10,
        op2: 32,
    };
    i.execute(&mut cpu);
    assert_eq!(cpu.rg_at_ref(10).expect("").get_i32(), -31);
}

#[test]
fn mul_test() {
    let mut cpu = Cpu::new(&String::from("HelloWorld.bin"));

    cpu.rg_at_mut(0).expect("").set_i32(32);
    cpu.rg_at_mut(1).expect("").set_i32(32);
    let mut i = Instruction {
        opcode: Opcode::Mul,
        op1: 0,
        op2: 1,
    };
    i.execute(&mut cpu);
    assert_eq!(cpu.rg_at_ref(0).expect("").get_i32(), 1024);

    cpu.rg_at_mut(10).expect("").set_i32(32);
    cpu.rg_at_mut(32).expect("").set_i32(-1000);
    let mut i = Instruction {
        opcode: Opcode::Mul,
        op1: 10,
        op2: 32,
    };
    i.execute(&mut cpu);
    assert_eq!(cpu.rg_at_ref(10).expect("").get_i32(), -32000);
}

#[test]
fn muli_test() {
    let mut cpu = Cpu::new(&String::from("HelloWorld.bin"));

    cpu.rg_at_mut(0).expect("").set_i32(32);
    let mut i = Instruction {
        opcode: Opcode::Muli,
        op1: 0,
        op2: 2,
    };
    i.execute(&mut cpu);
    assert_eq!(cpu.rg_at_ref(0).expect("").get_i32(), 64);

    cpu.rg_at_mut(10).expect("").set_i32(32);
    let mut i = Instruction {
        opcode: Opcode::Muli,
        op1: 10,
        op2: -1000,
    };
    i.execute(&mut cpu);
    assert_eq!(cpu.rg_at_ref(10).expect("").get_i32(), -32000);
}
