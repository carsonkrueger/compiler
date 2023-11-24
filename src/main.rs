use clap::Parser;
use lexer::Lexer;
mod lexer;
mod patterns;
mod token;
mod token_type;
mod visitor;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    file_path: String,
}

fn main() {
    let args = Args::parse();
    let mut lexer = Lexer::new(&args.file_path);

    for token in lexer {
        println!("{:?}", token);
    }
}
