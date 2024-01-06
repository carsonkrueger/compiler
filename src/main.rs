use crate::compiler::{lexer::Lexer, token::Token};
use clap::Parser;

mod asm;
mod compiler;
mod util;
mod vm;

#[derive(clap::Parser, Debug)]
struct Args {
    #[arg(short, long)]
    file_path: String,
}

fn main() {
    let args = Args::parse();

    let mut vm = vm::cpu::Cpu::new(&args.file_path);
    vm.run();

    // let mut lexer = Lexer::new(&args.file_path);
    // let tokens: Vec<Token> = lexer.into_iter().collect();

    // for t in &tokens {
    //     println!("{}", t.lexeme);
    // }

    // let parser = crate::compiler::parser::Parser::new(&tokens);

    // for expr in parser {
    //     println!("{:?}", expr);
    // }
}
