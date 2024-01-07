// use std::os::windows::fs::FileExt;
use clap::Parser;
use util::file_ext::file_ext;
use util::file_ext::FileExt;

#[allow(unused)]
mod asm;
#[allow(unused)]
mod compiler;
#[allow(unused)]
mod util;
#[allow(unused)]
mod vm;

#[derive(clap::Parser, Debug)]
struct Args {
    #[arg(short, long)]
    file_path: String,
}

fn main() {
    let args = Args::parse();

    let ext = match file_ext(&args.file_path) {
        Some(e) => e,
        None => panic!("Invalid file: {}", args.file_path),
    };

    match ext {
        // COMPILER
        FileExt::Txt => {
            let lexer = crate::compiler::lexer::Lexer::new(&args.file_path);
            let tokens: Vec<compiler::token::Token> = lexer.into_iter().collect();

            for t in &tokens {
                println!("{}", t.lexeme);
            }

            let parser = crate::compiler::parser::Parser::new(&tokens);

            for expr in parser {
                println!("{:?}", expr);
            }
        }
        // ASSEMBLER
        FileExt::Asm => {
            let asm_lexer = crate::asm::lexer::Lexer::new(&args.file_path);
            let asm_tokens: Vec<asm::token::Token> = asm_lexer.into_iter().collect();

            for t in &asm_tokens {
                println!("{}", t.lexeme);
            }
        }
        // VM
        FileExt::Bin => {
            let args = Args::parse();

            let mut vm = vm::cpu::Cpu::new(&args.file_path);
            vm.run();
        }
    };
}
