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
    let lexer = Lexer::new(&args.file_path);

    // if args.count != 1 {
    //     panic!("Invalid args");
    // }

    loop {
        let next_token = lexer.next();
        println!("{:?}", next_token);
        if next_token.is_none() {
            break;
        }
    }
}
