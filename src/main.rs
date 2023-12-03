use clap::Parser;
use lexer::Lexer;
use token::Token;

mod expressions;
mod lexer;
mod parser;
mod patterns;
mod statements;
mod token;
mod token_type;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    file_path: String,
}

fn main() {
    let args = Args::parse();

    let mut lexer = Lexer::new(&args.file_path);
    let tokens: Vec<Token> = lexer.into_iter().collect();
}
