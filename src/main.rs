mod lib;

use exitcode;
use lib::{parse_cli_args::CliArgs, remote_script, tokens::Token};
use logos::Logos;
use std::io::{self, Write};
use structopt::StructOpt;
use tokio::{self, fs};
use url::Url;

async fn cli() -> Result<(), Box<dyn std::error::Error>> {
	let args = CliArgs::from_args();

	if let Some(path) = args.path {
		if let Some(path) = path.to_str() {
			if let Ok(remote_path) = Url::parse(path) {
				let remote_script = remote_script::get_remote_script(remote_path.as_str()).await?;
				let lex = Token::lexer(&remote_script);

				for token in lex.spanned() {
					println!("{:?}", token);
				}
			} else {
				let local_script = fs::read_to_string(path).await?;
				let lex = Token::lexer(&local_script);

				for token in lex.spanned() {
					println!("{:?}", token);
				}
			}
		} else {
			eprintln!("There was an error in parsing the given file URL.");
			std::process::exit(exitcode::DATAERR);
		}
	} else if let Some(command_to_eval) = args.evaluate {
		let lex = Token::lexer(&command_to_eval);

		for token in lex.spanned() {
			println!("{:?}", token);
		}
	} else {
		loop {
			print!("> ");
			io::stdout().flush()?;

			let mut input = String::new();
			io::stdin().read_line(&mut input)?;

			let lex = Token::lexer(&input);
			for token in lex.spanned() {
				println!("{:?}", token);
			}
		}
	}

	Ok(())
}

#[tokio::main]
async fn main() {
	if cli().await.is_ok() {
		std::process::exit(exitcode::OK);
	} else {
		std::process::exit(exitcode::DATAERR);
	}
}
