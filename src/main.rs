extern crate logos;
extern crate structopt;
extern crate tokio;
mod libs;

#[allow(unused_imports)]
use libs::{file_reader, parse_cli_args::CliArgs, tokens::Token};
use logos::Logos;
use std::io::{self, Write};
use structopt::StructOpt;

#[tokio::main]
async fn main() {
    let args = CliArgs::from_args();

    match args.path {
        Some(path) => {
            let file_content = file_reader::read_content(path).await;
            let lex = Token::lexer(file_content.as_str());

            for token in lex.spanned() {
                println!("{:?}", token);
            }
        }
        None => loop {
            print!("> ");
            let _ = io::stdout().flush();

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Unable to read user input");

            let lex = Token::lexer(input.as_str());
            for token in lex.spanned() {
                println!("{:?}", token);
            }
        },
    }
}
