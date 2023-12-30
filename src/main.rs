use clap::Parser;
use lexer::Lexer;
use token::Token;

mod ast;
// mod expr;
mod lexer;
mod parser;
mod patterns;
// mod statements;
mod token;
mod visitor;

#[derive(clap::Parser, Debug)]
struct Args {
    #[arg(short, long)]
    file_path: String,
}

fn main() {
    let args = Args::parse();

    let mut lexer = Lexer::new(&args.file_path);
    let tokens: Vec<Token> = lexer.into_iter().collect();

    // for t in &tokens {
    //     println!("{}", t.lexeme);
    // }

    let parser = crate::parser::Parser::new(&tokens);

    for expr in parser {
        println!("{:?}", expr);
    }
}
