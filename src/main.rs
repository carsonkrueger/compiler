use clap::Parser;
use util::file_util::file_ext;
use util::file_util::file_name;
use util::file_util::FileExt;

#[allow(unused)]
mod compiler;
#[allow(unused)]
mod util;

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
        _ => panic!("Invalid file type {:?}", ext),
    };
}
