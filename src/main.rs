use clap::Parser;
use lexer::Lexer;
mod lexer;
mod patterns;
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

    lexer.run();
}
